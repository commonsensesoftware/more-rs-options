use crate::{OptionsChangeTokenSource, OptionsFactory, OptionsMonitorCache, Ref, Value};
use std::ops::Deref;
use std::sync::{Arc, RwLock, Weak};

/// Represents a change subscription.
///
/// # Remarks
///
/// When the subscription is dropped, the underlying callback is unsubscribed.
pub struct Subscription<T: Value>(Arc<dyn Fn(Option<&str>, Ref<T>) + Send + Sync>);

impl<T: Value> Subscription<T> {
    /// Initializes a new change token registration.
    pub fn new(callback: Arc<dyn Fn(Option<&str>, Ref<T>) + Send + Sync>) -> Self {
        Self(callback)
    }
}

unsafe impl<T: Send + Sync> Send for Subscription<T> {}
unsafe impl<T: Send + Sync> Sync for Subscription<T> {}

/// Defines the behavior for notifications when [`Options`](crate::Options) instances change.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait OptionsMonitor<T: Value> {
    /// Returns the current instance with the default options name.
    fn current_value(&self) -> Ref<T> {
        self.get(None)
    }

    /// Returns a configured instance with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name associated with the options.
    fn get(&self, name: Option<&str>) -> Ref<T>;

    /// Registers a callback function to be invoked when the configured instance with the given name changes.
    ///
    /// # Arguments
    ///
    /// * `listener` - The callback function to invoke
    ///
    /// # Returns
    ///
    /// A change subscription for the specified options. When the subscription is dropped, no further
    /// notifications will be propagated.
    fn on_change(
        &self,
        listener: Box<dyn Fn(Option<&str>, Ref<T>) + Send + Sync>,
    ) -> Subscription<T>;
}

/// Represents the default implementation for notifications when option instances change.
pub struct DefaultOptionsMonitor<T: Value> {
    tracker: Arc<ChangeTracker<T>>,
    _subscriptions: Vec<Box<dyn tokens::Subscription>>,
}

impl<T: Value + 'static> DefaultOptionsMonitor<T> {
    /// Initializes a new default options monitor.
    ///
    /// # Arguments
    ///
    /// * `cache` - The [cache](crate::OptionsMonitorCache) used for monitored options
    /// * `sources` - The [source tokens](crate::OptionsChangeTokenSource) used to track option changes
    /// * `factory` - The [factory](crate::OptionsFactory) used to create new options
    pub fn new(
        cache: Ref<dyn OptionsMonitorCache<T>>,
        sources: Vec<Ref<dyn OptionsChangeTokenSource<T>>>,
        factory: Ref<dyn OptionsFactory<T>>,
    ) -> Self {
        let tracker = Arc::new(ChangeTracker::new(cache, factory));
        let mut subscriptions = Vec::new();

        // SAFETY: the following is not guaranteed to be safe unless 'async' is enabled
        for source in sources {
            let producer = Producer::new(source.clone());
            let consumer = tracker.clone();
            let state = source.name().map(|n| Arc::new(n.to_owned()));
            let subscription: Box<dyn tokens::Subscription> = Box::new(tokens::on_change(
                move || producer.token(),
                move |state| {
                    if let Some(name) = state {
                        consumer.on_change(Some(name.as_str()));
                    } else {
                        consumer.on_change(None);
                    };
                },
                state,
            ));
            subscriptions.push(subscription);
        }

        Self {
            tracker,
            _subscriptions: subscriptions,
        }
    }
}

unsafe impl<T: Send + Sync> Send for DefaultOptionsMonitor<T> {}
unsafe impl<T: Send + Sync> Sync for DefaultOptionsMonitor<T> {}

impl<T: Value> OptionsMonitor<T> for DefaultOptionsMonitor<T> {
    fn get(&self, name: Option<&str>) -> Ref<T> {
        self.tracker.get(name)
    }

    fn on_change(
        &self,
        listener: Box<dyn Fn(Option<&str>, Ref<T>) + Send + Sync>,
    ) -> Subscription<T> {
        self.tracker.add(listener)
    }
}

struct ChangeTracker<T: Value> {
    cache: Ref<dyn OptionsMonitorCache<T>>,
    factory: Ref<dyn OptionsFactory<T>>,
    listeners: RwLock<Vec<Weak<dyn Fn(Option<&str>, Ref<T>) + Send + Sync>>>,
}

impl<T: Value> ChangeTracker<T> {
    fn new(cache: Ref<dyn OptionsMonitorCache<T>>, factory: Ref<dyn OptionsFactory<T>>) -> Self {
        Self {
            cache,
            factory,
            listeners: Default::default(),
        }
    }

    fn get(&self, name: Option<&str>) -> Ref<T> {
        self.cache
            .get_or_add(name, &|n| self.factory.create(n).unwrap())
    }

    fn add(&self, listener: Box<dyn Fn(Option<&str>, Ref<T>) + Send + Sync>) -> Subscription<T> {
        let mut listeners = self.listeners.write().unwrap();

        // writes are much infrequent and we already need to escalate
        // to a write-lock, so do the trimming of any dead callbacks now
        for i in (0..listeners.len()).rev() {
            if listeners[i].upgrade().is_none() {
                listeners.remove(i);
            }
        }

        let source: Arc<dyn Fn(Option<&str>, Ref<T>) + Send + Sync> = Arc::from(listener);

        listeners.push(Arc::downgrade(&source));
        Subscription::new(source)
    }

    fn on_change(&self, name: Option<&str>) {
        // acquire a read-lock and capture any callbacks that are still alive.
        // do NOT invoke the callback with the read-lock held. the callback might
        // register a new callback on the same token which will result in a deadlock.
        // invoking the callbacks after the read-lock is released ensures that won't happen.
        let callbacks: Vec<_> = self
            .listeners
            .read()
            .unwrap()
            .iter()
            .filter_map(|c| c.upgrade())
            .collect();

        self.cache.try_remove(name);

        for callback in callbacks {
            callback(name, self.get(name));
        }
    }
}

unsafe impl<T: Value> Send for ChangeTracker<T> {}
unsafe impl<T: Value> Sync for ChangeTracker<T> {}

struct Producer<T: Value>(Ref<dyn OptionsChangeTokenSource<T>>);

impl<T: Value> Producer<T> {
    fn new(source: Ref<dyn OptionsChangeTokenSource<T>>) -> Self {
        Self(source)
    }
}

impl<T: Value> Deref for Producer<T> {
    type Target = dyn OptionsChangeTokenSource<T>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

unsafe impl<T: Value> Send for Producer<T> {}
unsafe impl<T: Value> Sync for Producer<T> {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;
    use std::{
        cell::RefCell,
        sync::atomic::{AtomicBool, AtomicU8, Ordering},
    };
    use tokens::{ChangeToken, SharedChangeToken, SingleChangeToken};

    #[derive(Default)]
    struct Config {
        retries: u8,
    }

    pub struct OptionsState {
        dirty: AtomicBool,
    }

    impl OptionsState {
        fn is_dirty(&self) -> bool {
            self.dirty.load(Ordering::SeqCst)
        }

        fn mark_dirty(&self) {
            self.dirty.store(true, Ordering::SeqCst)
        }

        fn reset(&self) {
            self.dirty.store(false, Ordering::SeqCst)
        }
    }

    impl Default for OptionsState {
        fn default() -> Self {
            Self {
                dirty: AtomicBool::new(true),
            }
        }
    }

    #[derive(Default)]
    struct ConfigSetup {
        counter: AtomicU8,
    }

    impl ConfigureOptions<Config> for ConfigSetup {
        fn configure(&self, name: Option<&str>, options: &mut Config) {
            if name.is_none() {
                let retries = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
                options.retries = retries;
            }
        }
    }

    #[derive(Default)]
    struct ConfigSource {
        token: SharedChangeToken<SingleChangeToken>,
    }

    impl ConfigSource {
        fn changed(&self) {
            self.token.notify()
        }
    }

    impl OptionsChangeTokenSource<Config> for ConfigSource {
        fn token(&self) -> Box<dyn ChangeToken> {
            Box::new(self.token.clone())
        }
    }

    struct Foo {
        monitor: Ref<dyn OptionsMonitor<Config>>,
        _sub: Subscription<Config>,
        state: Arc<OptionsState>,
        retries: RefCell<u8>,
    }

    impl Foo {
        fn new(monitor: Ref<dyn OptionsMonitor<Config>>) -> Self {
            let state = Arc::new(OptionsState::default());
            let other = state.clone();

            Self {
                monitor: monitor.clone(),
                _sub: monitor.on_change(Box::new(
                    move |_name: Option<&str>, _options: Ref<Config>| other.mark_dirty(),
                )),
                state,
                retries: RefCell::default(),
            }
        }

        fn retries(&self) -> u8 {
            if self.state.is_dirty() {
                *self.retries.borrow_mut() = self.monitor.current_value().retries;
                self.state.reset();
            }

            self.retries.borrow().clone()
        }
    }

    #[test]
    fn monitored_options_should_update_when_source_changes() {
        // arrange
        let cache = Ref::new(OptionsCache::<Config>::default());
        let setup = Ref::new(ConfigSetup::default());
        let factory = Ref::new(DefaultOptionsFactory::new(
            vec![setup],
            Vec::default(),
            Vec::default(),
        ));
        let source = Ref::new(ConfigSource::default());
        let monitor = Ref::new(DefaultOptionsMonitor::new(
            cache,
            vec![source.clone()],
            factory,
        ));
        let foo = Foo::new(monitor);
        let initial = foo.retries();

        // act
        source.changed();

        // assert
        assert_eq!(initial, 1);
        assert_eq!(foo.retries(), 2);
    }
}

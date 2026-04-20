use crate::{OptionsChangeTokenSource, OptionsFactory, OptionsMonitorCache, Ref, Value};
use cfg_if::cfg_if;
use std::sync::{Arc, RwLock, Weak};

cfg_if! {
    if #[cfg(not(feature = "async"))] {
        use std::cell::RefCell;
        use tokens::ChangeToken;
    }
}

type Callback<T> = dyn Fn(Option<&str>, Ref<T>) + Send + Sync;

/// Represents a change subscription.
///
/// # Remarks
///
/// When the subscription is dropped, the underlying callback is unsubscribed.
pub struct Subscription<T: Value>(#[allow(unused)] Arc<Callback<T>>);

impl<T: Value> Subscription<T> {
    /// Initializes a new change token registration.
    ///
    /// # Arguments
    ///
    /// * `callback` - The subscription callback function
    #[inline]
    pub fn new(callback: Arc<Callback<T>>) -> Self {
        Self(callback)
    }
}

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
    /// * `changed` - The callback function to invoke
    ///
    /// # Returns
    ///
    /// A change subscription for the specified options. When the subscription is dropped, no further
    /// notifications will be propagated.
    fn on_change(&self, changed: Box<Callback<T>>) -> Subscription<T>;
}

/// Represents the default implementation for notifications when option instances change.
pub struct DefaultOptionsMonitor<T: Value> {
    tracker: Arc<ChangeTracker<T>>,
    _subscriptions: Vec<Box<dyn tokens::Subscription>>,
}

#[cfg(feature = "async")]
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

#[cfg(not(feature = "async"))]
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
        Self {
            tracker: Arc::new(ChangeTracker::new(cache, sources, factory)),
            _subscriptions: Vec::new(),
        }
    }
}

impl<T: Value> OptionsMonitor<T> for DefaultOptionsMonitor<T> {
    fn get(&self, name: Option<&str>) -> Ref<T> {
        cfg_if! {
            if #[cfg(not(feature = "async"))] {
                self.tracker.check_for_changes();
            }
        }

        self.tracker.get(name)
    }

    #[inline]
    fn on_change(&self, changed: Box<Callback<T>>) -> Subscription<T> {
        self.tracker.add(changed)
    }
}

struct ChangeTracker<T: Value> {
    cache: Ref<dyn OptionsMonitorCache<T>>,
    factory: Ref<dyn OptionsFactory<T>>,
    listeners: RwLock<Vec<Weak<Callback<T>>>>,

    #[cfg(not(feature = "async"))]
    sources: Vec<Ref<dyn OptionsChangeTokenSource<T>>>,

    #[cfg(not(feature = "async"))]
    tokens: RefCell<Vec<Box<dyn ChangeToken>>>,

    /// tracks whether each source's current token change has already been processed, which prevents re-firing when
    /// `source.token()` returns a token that is already in the "changed" state
    #[cfg(not(feature = "async"))]
    processed: RefCell<Vec<bool>>,
}

impl<T: Value> ChangeTracker<T> {
    fn get(&self, name: Option<&str>) -> Ref<T> {
        self.cache
            .get_or_add(name, &|n| self.factory.create(n).unwrap_or_else(|e| panic!("{}", e)))
    }

    fn add(&self, listener: Box<Callback<T>>) -> Subscription<T> {
        let mut listeners = self.listeners.write().unwrap();

        // trim any dead callbacks while holding the write-lock
        for i in (0..listeners.len()).rev() {
            if listeners[i].upgrade().is_none() {
                listeners.remove(i);
            }
        }

        let source: Arc<Callback<T>> = Arc::from(listener);

        listeners.push(Arc::downgrade(&source));
        Subscription::new(source)
    }

    fn on_change(&self, name: Option<&str>) {
        // acquire a read-lock and capture any callbacks that are still alive. do NOT invoke the callback with the
        // read-lock held. the callback might register a new callback on the same token which will result in a deadlock.
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

#[cfg(feature = "async")]
impl<T: Value> ChangeTracker<T> {
    #[inline]
    fn new(cache: Ref<dyn OptionsMonitorCache<T>>, factory: Ref<dyn OptionsFactory<T>>) -> Self {
        Self {
            cache,
            factory,
            listeners: Default::default(),
        }
    }
}

#[cfg(not(feature = "async"))]
impl<T: Value> ChangeTracker<T> {
    fn new(
        cache: Ref<dyn OptionsMonitorCache<T>>,
        sources: Vec<Ref<dyn OptionsChangeTokenSource<T>>>,
        factory: Ref<dyn OptionsFactory<T>>,
    ) -> Self {
        let len = sources.len();
        let tokens = sources.iter().map(|s| s.token()).collect();
        Self {
            cache,
            factory,
            listeners: Default::default(),
            sources,
            tokens: RefCell::new(tokens),
            processed: RefCell::new(vec![false; len]),
        }
    }

    fn check_for_changes(&self) {
        let mut tokens = self.tokens.borrow_mut();
        let mut processed = self.processed.borrow_mut();

        for (i, source) in self.sources.iter().enumerate() {
            if tokens[i].changed() && !processed[i] {
                self.on_change(source.name());

                let new_token = source.token();

                // if the new token is already changed (e.g. SharedChangeToken shares state with the old one), mark it
                // as processed so we don't re-fire on the next check_for_changes() call.
                processed[i] = new_token.changed();
                tokens[i] = new_token;
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "async")] {
        struct Producer<T: Value>(Ref<dyn OptionsChangeTokenSource<T>>);

        impl<T: Value> Producer<T> {
            #[inline]
            fn new(source: Ref<dyn OptionsChangeTokenSource<T>>) -> Self {
                Self(source)
            }
        }

        impl<T: Value> std::ops::Deref for Producer<T> {
            type Target = dyn OptionsChangeTokenSource<T>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.0.deref()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::{
        cell::RefCell,
        sync::{
            atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering},
            Mutex,
        },
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
                _sub: monitor.on_change(Box::new(move |_name: Option<&str>, _options: Ref<Config>| {
                    other.mark_dirty()
                })),
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

    fn new_monitor() -> (Ref<dyn OptionsMonitor<Config>>, Ref<ConfigSource>, Ref<ConfigSetup>) {
        let cache = Ref::new(OptionsCache::<Config>::default());
        let setup = Ref::new(ConfigSetup::default());
        let factory = Ref::new(DefaultOptionsFactory::new(
            vec![setup.clone()],
            Vec::default(),
            Vec::default(),
        ));
        let source = Ref::new(ConfigSource::default());
        let monitor: Ref<dyn OptionsMonitor<Config>> =
            Ref::new(DefaultOptionsMonitor::new(cache, vec![source.clone()], factory));
        (monitor, source, setup)
    }

    /// A named [OptionsChangeTokenSource] for testing multi-source scenarios.
    struct NamedConfigSource {
        name: String,
        token: SharedChangeToken<SingleChangeToken>,
    }

    impl NamedConfigSource {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
                token: SharedChangeToken::default(),
            }
        }

        fn changed(&self) {
            self.token.notify()
        }
    }

    impl OptionsChangeTokenSource<Config> for NamedConfigSource {
        fn token(&self) -> Box<dyn ChangeToken> {
            Box::new(self.token.clone())
        }

        fn name(&self) -> Option<&str> {
            Some(&self.name)
        }
    }

    /// A [ConfigureOptions] that sets retries based on name using an atomic counter per name to track how many times
    /// the factory has been called.
    struct NamedConfigSetup {
        a: AtomicU8,
        b: AtomicU8,
    }

    impl Default for NamedConfigSetup {
        fn default() -> Self {
            Self {
                a: AtomicU8::new(0),
                b: AtomicU8::new(0),
            }
        }
    }

    impl ConfigureOptions<Config> for NamedConfigSetup {
        fn configure(&self, name: Option<&str>, options: &mut Config) {
            match name {
                Some("a") => {
                    options.retries = self.a.fetch_add(1, Ordering::SeqCst) + 10;
                }
                Some("b") => {
                    options.retries = self.b.fetch_add(1, Ordering::SeqCst) + 20;
                }
                _ => {}
            }
        }
    }

    #[test]
    fn monitored_options_should_update_when_source_changes() {
        // arrange
        let cache = Ref::new(OptionsCache::<Config>::default());
        let setup = Ref::new(ConfigSetup::default());
        let factory = Ref::new(DefaultOptionsFactory::new(vec![setup], Vec::default(), Vec::default()));
        let source = Ref::new(ConfigSource::default());
        let monitor: Ref<dyn OptionsMonitor<Config>> =
            Ref::new(DefaultOptionsMonitor::new(cache, vec![source.clone()], factory));
        let foo = Foo::new(monitor.clone());
        let initial = foo.retries();

        // act
        source.changed();

        let _ = monitor.get(None);

        // assert
        assert_eq!(initial, 1);
        assert_eq!(foo.retries(), 2);
    }

    #[test]
    fn get_none_returns_factory_created_value() {
        // arrange
        let (monitor, source, _setup) = new_monitor();

        let first = monitor.get(None);
        assert_eq!(first.retries, 1);

        let cached = monitor.get(None);
        assert_eq!(cached.retries, 1);

        // act
        source.changed();

        // assert
        let updated = monitor.get(None);

        assert_eq!(updated.retries, 2);
    }

    #[test]
    fn on_change_callbacks_fire_with_correct_name_and_value() {
        // arrange
        let (monitor, source, _setup) = new_monitor();
        let _ = monitor.get(None);
        let observed_name: Arc<Mutex<Option<Option<String>>>> = Arc::new(Mutex::new(None));
        let observed_retries: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
        let name_clone = observed_name.clone();
        let retries_clone = observed_retries.clone();

        let _sub = monitor.on_change(Box::new(move |name: Option<&str>, opts: Ref<Config>| {
            *name_clone.lock().unwrap() = Some(name.map(|s| s.to_owned()));
            *retries_clone.lock().unwrap() = Some(opts.retries);
        }));

        // act
        source.changed();
        let _ = monitor.get(None);

        // assert
        let name_val = observed_name.lock().unwrap();

        assert_eq!(
            *name_val,
            Some(None),
            "callback should receive name=None for unnamed source"
        );

        let retries_val = observed_retries.lock().unwrap();

        assert_eq!(*retries_val, Some(2), "callback should receive updated retries value");
    }

    #[test]
    fn dropping_subscription_prevents_further_callbacks() {
        // arrange
        let (monitor, source, _setup) = new_monitor();
        let _ = monitor.get(None);
        let call_count = Arc::new(AtomicU32::new(0));
        let count_clone = call_count.clone();
        let sub = monitor.on_change(Box::new(move |_name: Option<&str>, _opts: Ref<Config>| {
            count_clone.fetch_add(1, Ordering::SeqCst);
        }));

        // act
        source.changed();
        let _ = monitor.get(None);

        assert_eq!(
            call_count.load(Ordering::SeqCst),
            1,
            "callback should fire once after first change"
        );

        drop(sub);

        // act
        source.changed();
        let _ = monitor.get(None);

        // assert
        assert_eq!(
            call_count.load(Ordering::SeqCst),
            1,
            "callback should not fire after subscription is dropped"
        );
    }

    #[test]
    fn multiple_sources_changing_one_only_invalidates_that_source() {
        // arrange
        let cache = Ref::new(OptionsCache::<Config>::default());
        let setup = Ref::new(NamedConfigSetup::default());
        let factory = Ref::new(DefaultOptionsFactory::new(vec![setup], Vec::default(), Vec::default()));
        let source_a = Ref::new(NamedConfigSource::new("a"));
        let source_b = Ref::new(NamedConfigSource::new("b"));
        let monitor: Ref<dyn OptionsMonitor<Config>> = Ref::new(DefaultOptionsMonitor::new(
            cache,
            vec![source_a.clone(), source_b.clone()],
            factory,
        ));
        let val_a = monitor.get(Some("a"));
        let val_b = monitor.get(Some("b"));

        assert_eq!(val_a.retries, 10, "source a initial retries");
        assert_eq!(val_b.retries, 20, "source b initial retries");

        let callback_names: Arc<Mutex<Vec<Option<String>>>> = Arc::new(Mutex::new(Vec::new()));
        let names_clone = callback_names.clone();
        let _sub = monitor.on_change(Box::new(move |name: Option<&str>, _opts: Ref<Config>| {
            names_clone.lock().unwrap().push(name.map(|s| s.to_owned()));
        }));

        // act
        source_a.changed();

        let _ = monitor.get(Some("a"));

        // assert
        let names = callback_names.lock().unwrap();
        assert_eq!(names.len(), 1, "only one callback should fire");
        assert_eq!(names[0], Some("a".to_owned()), "callback should fire for source a");
        drop(names);

        let val_a_updated = monitor.get(Some("a"));
        assert_eq!(val_a_updated.retries, 11, "source a should be invalidated → new value");

        let val_b_same = monitor.get(Some("b"));
        assert_eq!(val_b_same.retries, 20, "source b should still be cached → same value");
    }
}

use crate::{OptionsChangeTokenSource, OptionsFactory, OptionsMonitorCache, Ref};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::Mutex;
use tokens::{ChangeToken, NeverChangeToken};

/// Defines the behavior for notifications when option instances change.
pub trait OptionsMonitor<T> {
    /// Returns the current instance with the default options name.
    fn current_value(&self) -> &T;

    /// Returns a configured instance with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - the name associated with the options.
    fn get(&self, name: Option<&str>) -> &T;

    /// Registers a callback function to be invoked when the configured instance with the given name changes.
    ///
    /// # Arguments
    ///
    /// * `listener` - a weak reference to callback function to invoke
    fn on_change(&self, listener: Weak<dyn Fn(Option<&str>, &T)>);
}

/// Represents the default implementation for notifications when option instances change.
pub struct DefaultOptionsMonitor<T> {
    mediator: Rc<Mediator<T>>,
}

impl<T: 'static> DefaultOptionsMonitor<T> {
    /// Initializes a new default options monitor.
    ///
    /// # Arguments
    ///
    /// * `cache` - the [cache](trait.OptionsMonitorCache.html) used for monitored options
    /// * `sources` - the [source tokens](trait.OptionsChangeToken.html) used to track option changes
    /// * `factory` - the [factory](trait.OptionsFactory.html) used to create new options
    pub fn new(
        cache: Ref<dyn OptionsMonitorCache<T>>,
        sources: Vec<Ref<dyn OptionsChangeTokenSource<T>>>,
        factory: Ref<dyn OptionsFactory<T>>,
    ) -> Self {
        let mediator = Rc::new(Mediator::new(cache, factory));
        let instance = Self {
            mediator: mediator.clone(),
        };
        let registrations = sources
            .into_iter()
            .map(|producer| ChangeTokenRegistration::new(producer, mediator.clone()));

        mediator.mediate(registrations);
        instance
    }
}

impl<T> OptionsMonitor<T> for DefaultOptionsMonitor<T> {
    fn current_value(&self) -> &T {
        self.get(None)
    }

    fn get(&self, name: Option<&str>) -> &T {
        self.mediator.get(name)
    }

    fn on_change(&self, listener: Weak<dyn Fn(Option<&str>, &T)>) {
        self.mediator.on_change(listener)
    }
}

struct Mediator<T> {
    cache: Ref<dyn OptionsMonitorCache<T>>,
    factory: Ref<dyn OptionsFactory<T>>,
    registrations: RefCell<Vec<Rc<ChangeTokenRegistration<T>>>>,
    listeners: Mutex<Vec<Weak<dyn Fn(Option<&str>, &T)>>>,
}

impl<T> Mediator<T> {
    fn new(cache: Ref<dyn OptionsMonitorCache<T>>, factory: Ref<dyn OptionsFactory<T>>) -> Self {
        Self {
            cache,
            factory,
            registrations: RefCell::default(),
            listeners: Mutex::default(),
        }
    }

    fn mediate(&self, registrations: impl Iterator<Item = Rc<ChangeTokenRegistration<T>>>) {
        let mut collection = self.registrations.borrow_mut();

        for registration in registrations {
            collection.push(registration);
        }
    }

    fn changed(&self, name: Option<&str>) {
        self.cache.try_remove(name);

        let options = self.get(name);
        let mut listeners = self.listeners.lock().unwrap();

        for i in (0..listeners.len()).rev() {
            if let Some(callback) = listeners[i].upgrade() {
                callback(name, options);
            } else {
                listeners.remove(i);
            }
        }
    }

    fn get(&self, name: Option<&str>) -> &T {
        self.cache
            .get_or_add(name, &|n| self.factory.create(n).unwrap())
    }

    fn on_change(&self, listener: Weak<dyn Fn(Option<&str>, &T)>) {
        self.listeners.lock().unwrap().push(listener);
    }
}

struct ChangeTokenRegistration<T> {
    producer: Ref<dyn OptionsChangeTokenSource<T>>,
    consumer: Rc<Mediator<T>>,
    token: RefCell<Box<dyn ChangeToken>>,
    me: Weak<ChangeTokenRegistration<T>>,
}

impl<T: 'static> ChangeTokenRegistration<T> {
    fn new(producer: Ref<dyn OptionsChangeTokenSource<T>>, consumer: Rc<Mediator<T>>) -> Rc<Self> {
        let instance = Rc::new_cyclic(|me| Self {
            producer,
            consumer,
            token: RefCell::new(Box::new(NeverChangeToken::new())),
            me: me.clone(),
        });
        let token = instance.producer.token();
        instance.register(token);
        return instance;
    }

    fn register(&self, token: Box<dyn ChangeToken>) {
        let sender = self.me.clone();
        token.register(Box::new(move || {
            ChangeTokenRegistration::<T>::fire(sender.clone());
        }));
        self.token.replace(token);
    }

    fn fire(sender: Weak<ChangeTokenRegistration<T>>) {
        let me = sender.upgrade().unwrap();
        me.consumer.changed(me.producer.name());
        me.register(me.producer.token());
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;
    use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
    use tokens::SingleChangeToken;

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
        triggers: RefCell<Vec<Weak<dyn Fn()>>>,
    }

    impl ConfigSource {
        fn changed(&self) {
            let mut triggers = self.triggers.replace(Vec::default());

            for trigger in triggers.drain(..) {
                if let Some(callback) = trigger.upgrade() {
                    (callback)()
                }
            }
        }
    }

    impl OptionsChangeTokenSource<Config> for ConfigSource {
        fn token(&self) -> Box<dyn ChangeToken> {
            let token = Box::new(SingleChangeToken::default());

            loop {
                if let Ok(mut triggers) = self.triggers.try_borrow_mut() {
                    triggers.push(token.trigger());
                    break;
                }
            }

            token
        }
    }

    struct Foo {
        monitor: Ref<dyn OptionsMonitor<Config>>,
        handler: Rc<dyn Fn(Option<&str>, &Config)>,
        state: Rc<OptionsState>,
        retries: RefCell<u8>,
    }

    impl Foo {
        fn new(monitor: Ref<dyn OptionsMonitor<Config>>) -> Self {
            let state = Rc::new(OptionsState::default());
            let other_state = state.clone();
            let instance = Self {
                monitor: monitor.clone(),
                handler: Rc::new(move |_name: Option<&str>, _options: &Config| {
                    other_state.mark_dirty()
                }),
                state,
                retries: RefCell::default(),
            };

            monitor.on_change(Rc::downgrade(&instance.handler.clone()));
            instance
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

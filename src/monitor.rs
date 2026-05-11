use crate::{validation::Error, Cache, ChangeTokenSource, Factory, Ref, Value};
use cfg_if::cfg_if;
use std::sync::{Arc, RwLock, Weak};

cfg_if! {
    if #[cfg(not(feature = "async"))] {
        use std::cell::RefCell;
        use tokens::ChangeToken;
    }
}

type Callback<T> = dyn Fn(&str, Ref<T>) + Send + Sync;

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

/// Defines the behavior for notifications when options instances change.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait Monitor<T: Value> {
    /// Gets the default, unnamed configuration options.
    fn get(&self) -> Result<Ref<T>, Error> {
        self.get_named("")
    }

    /// Gets the default, unnamed configuration options.
    ///
    /// # Remarks
    ///
    /// This function panics if the configuration options could not be successfully retrieved.
    fn get_unchecked(&self) -> Ref<T> {
        match self.get_named("") {
            Ok(value) => value,
            Err(error) => panic!("{}", error),
        }
    }

    /// Gets the configuration options with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name associated with the options.
    fn get_named(&self, name: &str) -> Result<Ref<T>, Error>;

    /// Gets the configuration options with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to retrieve
    ///
    /// # Remarks
    ///
    /// This function panics if the configuration options could not be successfully retrieved.
    fn get_named_unchecked(&self, name: &str) -> Ref<T> {
        match self.get_named(name) {
            Ok(value) => value,
            Err(error) => panic!("[{name}] {}", error),
        }
    }

    /// Registers a callback function to be invoked when the configured instance with the given name changes.
    ///
    /// # Arguments
    ///
    /// * `changed` - The callback function to invoke
    ///
    /// # Returns
    ///
    /// A change subscription for the specified options. When the subscription is dropped, no further notifications
    /// will be propagated.
    #[must_use = "no change notifications occur after the subscription is dropped"]
    fn on_change(&self, changed: Box<Callback<T>>) -> Subscription<T>;
}

/// Represents the default implementation for notifications when option instances change.
pub struct DefaultMonitor<T: Value> {
    tracker: Arc<ChangeTracker<T>>,
    _subscriptions: Vec<Box<dyn tokens::Subscription>>,
}

#[cfg(feature = "async")]
impl<T: Value + 'static> DefaultMonitor<T> {
    /// Initializes a new default options monitor.
    ///
    /// # Arguments
    ///
    /// * `cache` - The [cache](crate::Cache) used for monitored options
    /// * `sources` - The [source tokens](crate::ChangeTokenSource) used to track option changes
    /// * `factory` - The [factory](crate::Factory) used to create new options
    pub fn new(
        cache: Ref<Cache<T>>,
        sources: Vec<Ref<dyn ChangeTokenSource<T>>>,
        factory: Ref<dyn Factory<T>>,
    ) -> Self {
        let tracker = Arc::new(ChangeTracker::new(cache, factory));
        let mut subscriptions = Vec::new();

        for source in sources {
            let producer = Producer::new(source.clone());
            let consumer = tracker.clone();
            let state = Arc::new(source.name().to_owned());
            let subscription: Box<dyn tokens::Subscription> = Box::new(tokens::on_change(
                move || producer.token(),
                move |state| {
                    if let Some(name) = state {
                        consumer.on_change(&name);
                    } else {
                        consumer.on_change("");
                    };
                },
                Some(state),
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
impl<T: Value + 'static> DefaultMonitor<T> {
    /// Initializes a new default options monitor.
    ///
    /// # Arguments
    ///
    /// * `cache` - The [cache](crate::Cache) used for monitored options
    /// * `sources` - The [source tokens](crate::ChangeTokenSource) used to track option changes
    /// * `factory` - The [factory](crate::Factory) used to create new options
    pub fn new(
        cache: Ref<Cache<T>>,
        sources: Vec<Ref<dyn ChangeTokenSource<T>>>,
        factory: Ref<dyn Factory<T>>,
    ) -> Self {
        Self {
            tracker: Arc::new(ChangeTracker::new(cache, sources, factory)),
            _subscriptions: Vec::new(),
        }
    }
}

impl<T: Value> Monitor<T> for DefaultMonitor<T> {
    fn get_named(&self, name: &str) -> Result<Ref<T>, Error> {
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
    cache: Ref<Cache<T>>,
    factory: Ref<dyn Factory<T>>,
    listeners: RwLock<Vec<Weak<Callback<T>>>>,

    #[cfg(not(feature = "async"))]
    sources: Vec<Ref<dyn ChangeTokenSource<T>>>,

    #[cfg(not(feature = "async"))]
    tokens: RefCell<Vec<Box<dyn ChangeToken>>>,

    /// tracks whether each source's current token change has already been processed, which prevents re-firing when
    /// `source.token()` returns a token that is already in the "changed" state
    #[cfg(not(feature = "async"))]
    processed: RefCell<Vec<bool>>,
}

impl<T: Value> ChangeTracker<T> {
    fn get(&self, name: &str) -> Result<Ref<T>, Error> {
        self.cache.get_or_add(name, &|n| self.factory.create(n))
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

    fn on_change(&self, name: &str) {
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
            if let Ok(options) = self.get(name) {
                callback(name, options);
            }
        }
    }
}

#[cfg(feature = "async")]
impl<T: Value> ChangeTracker<T> {
    #[inline]
    fn new(cache: Ref<Cache<T>>, factory: Ref<dyn Factory<T>>) -> Self {
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
        cache: Ref<Cache<T>>,
        sources: Vec<Ref<dyn ChangeTokenSource<T>>>,
        factory: Ref<dyn Factory<T>>,
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
        struct Producer<T: Value>(Ref<dyn ChangeTokenSource<T>>);

        impl<T: Value> Producer<T> {
            #[inline]
            fn new(source: Ref<dyn ChangeTokenSource<T>>) -> Self {
                Self(source)
            }
        }

        impl<T: Value> std::ops::Deref for Producer<T> {
            type Target = dyn ChangeTokenSource<T>;

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
    use crate::{Cache, Configure, DefaultFactory};
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
        #[inline]
        fn is_dirty(&self) -> bool {
            self.dirty.load(Ordering::SeqCst)
        }

        #[inline]
        fn mark_dirty(&self) {
            self.dirty.store(true, Ordering::SeqCst)
        }

        #[inline]
        fn reset(&self) {
            self.dirty.store(false, Ordering::SeqCst)
        }
    }

    impl Default for OptionsState {
        #[inline]
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

    impl Configure<Config> for ConfigSetup {
        fn run(&self, name: &str, options: &mut Config) {
            if name.is_empty() {
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
        #[inline]
        fn changed(&self) {
            self.token.notify()
        }
    }

    impl ChangeTokenSource<Config> for ConfigSource {
        #[inline]
        fn token(&self) -> Box<dyn ChangeToken> {
            Box::new(self.token.clone())
        }
    }

    struct Foo {
        monitor: Ref<dyn Monitor<Config>>,
        _sub: Subscription<Config>,
        state: Arc<OptionsState>,
        retries: RefCell<u8>,
    }

    impl Foo {
        fn new(monitor: Ref<dyn Monitor<Config>>) -> Self {
            let state = Arc::new(OptionsState::default());
            let other = state.clone();

            Self {
                monitor: monitor.clone(),
                _sub: monitor.on_change(Box::new(move |_name: &str, _options: Ref<Config>| other.mark_dirty())),
                state,
                retries: RefCell::default(),
            }
        }

        fn retries(&self) -> u8 {
            if self.state.is_dirty() {
                *self.retries.borrow_mut() = self.monitor.get_unchecked().retries;
                self.state.reset();
            }

            self.retries.borrow().clone()
        }
    }

    fn new_monitor() -> (Ref<dyn Monitor<Config>>, Ref<ConfigSource>, Ref<ConfigSetup>) {
        let cache = Ref::new(Cache::<Config>::default());
        let setup = Ref::new(ConfigSetup::default());
        let factory = Ref::new(DefaultFactory::new(vec![setup.clone()], Vec::default(), Vec::default()));
        let source = Ref::new(ConfigSource::default());
        let monitor: Ref<dyn Monitor<Config>> = Ref::new(DefaultMonitor::new(cache, vec![source.clone()], factory));
        (monitor, source, setup)
    }

    /// A named [ChangeTokenSource] for testing multi-source scenarios.
    struct NamedConfigSource {
        name: String,
        token: SharedChangeToken<SingleChangeToken>,
    }

    impl NamedConfigSource {
        #[inline]
        fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
                token: SharedChangeToken::default(),
            }
        }

        #[inline]
        fn changed(&self) {
            self.token.notify()
        }
    }

    impl ChangeTokenSource<Config> for NamedConfigSource {
        #[inline]
        fn token(&self) -> Box<dyn ChangeToken> {
            Box::new(self.token.clone())
        }

        #[inline]
        fn name(&self) -> &str {
            &self.name
        }
    }

    /// A [Configure] that sets retries based on name using an atomic counter per name to track how many times
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

    impl Configure<Config> for NamedConfigSetup {
        fn run(&self, name: &str, options: &mut Config) {
            match name {
                "a" => options.retries = self.a.fetch_add(1, Ordering::SeqCst) + 10,
                "b" => options.retries = self.b.fetch_add(1, Ordering::SeqCst) + 20,
                _ => {}
            }
        }
    }

    #[test]
    fn monitored_options_should_update_when_source_changes() {
        // arrange
        let cache = Ref::new(Cache::<Config>::default());
        let setup = Ref::new(ConfigSetup::default());
        let factory = Ref::new(DefaultFactory::new(vec![setup], Vec::default(), Vec::default()));
        let source = Ref::new(ConfigSource::default());
        let monitor: Ref<dyn Monitor<Config>> = Ref::new(DefaultMonitor::new(cache, vec![source.clone()], factory));
        let foo = Foo::new(monitor.clone());
        let initial = foo.retries();

        // act
        source.changed();

        let _ = monitor.get_unchecked();

        // assert
        assert_eq!(initial, 1);
        assert_eq!(foo.retries(), 2);
    }

    #[test]
    fn get_none_returns_factory_created_value() {
        // arrange
        let (monitor, source, _) = new_monitor();

        let first = monitor.get_unchecked();
        assert_eq!(first.retries, 1);

        let cached = monitor.get_unchecked();
        assert_eq!(cached.retries, 1);

        // act
        source.changed();

        // assert
        let updated = monitor.get_unchecked();

        assert_eq!(updated.retries, 2);
    }

    #[test]
    fn on_change_callbacks_fire_with_correct_name_and_value() {
        // arrange
        let (monitor, source, _) = new_monitor();
        let _ = monitor.get();
        let observed_name: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
        let observed_retries: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
        let name_clone = observed_name.clone();
        let retries_clone = observed_retries.clone();
        let _sub = monitor.on_change(Box::new(move |name, opts| {
            *name_clone.lock().unwrap() = name.to_owned();
            *retries_clone.lock().unwrap() = opts.retries;
        }));

        // act
        source.changed();
        let _ = monitor.get();

        // assert
        let name_val = observed_name.lock().unwrap();

        assert_eq!(*name_val, "", "callback should receive '' for unnamed source");

        let retries_val = observed_retries.lock().unwrap();

        assert_eq!(*retries_val, 2, "callback should receive updated retries value");
    }

    #[test]
    fn dropping_subscription_prevents_further_callbacks() {
        // arrange
        let (monitor, source, _setup) = new_monitor();
        let _ = monitor.get();
        let call_count = Arc::new(AtomicU32::new(0));
        let count_clone = call_count.clone();
        let sub = monitor.on_change(Box::new(move |_, _| {
            count_clone.fetch_add(1, Ordering::SeqCst);
        }));

        // act
        source.changed();
        let _ = monitor.get();

        assert_eq!(
            call_count.load(Ordering::SeqCst),
            1,
            "callback should fire once after first change"
        );

        drop(sub);

        // act
        source.changed();
        let _ = monitor.get();

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
        let cache = Ref::new(Cache::<Config>::default());
        let setup = Ref::new(NamedConfigSetup::default());
        let factory = Ref::new(DefaultFactory::new(vec![setup], Vec::default(), Vec::default()));
        let source_a = Ref::new(NamedConfigSource::new("a"));
        let source_b = Ref::new(NamedConfigSource::new("b"));
        let monitor: Ref<dyn Monitor<Config>> = Ref::new(DefaultMonitor::new(
            cache,
            vec![source_a.clone(), source_b.clone()],
            factory,
        ));
        let val_a = monitor.get_named_unchecked("a");
        let val_b = monitor.get_named_unchecked("b");

        assert_eq!(val_a.retries, 10, "source a initial retries");
        assert_eq!(val_b.retries, 20, "source b initial retries");

        let callback_names: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let names_clone = callback_names.clone();
        let _sub = monitor.on_change(Box::new(move |name, _| {
            names_clone.lock().unwrap().push(name.to_owned());
        }));

        // act
        source_a.changed();

        let _ = monitor.get_named("a");

        // assert
        let names = callback_names.lock().unwrap();
        assert_eq!(names.len(), 1, "only one callback should fire");
        assert_eq!(names[0], "a", "callback should fire for source a");
        drop(names);

        let val_a_updated = monitor.get_named_unchecked("a");
        assert_eq!(val_a_updated.retries, 11, "source a should be invalidated → new value");

        let val_b_same = monitor.get_named_unchecked("b");
        assert_eq!(val_b_same.retries, 20, "source b should still be cached → same value");
    }
}

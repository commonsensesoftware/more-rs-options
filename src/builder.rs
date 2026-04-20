use crate::{ConfigureOptions, PostConfigureOptions, Ref, ValidateOptions, ValidateOptionsResult, Value};
use cfg_if::cfg_if;
use di::{singleton_factory, transient_factory, Ref as Svc, ServiceCollection};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Represents a builder used to configure [`Options`](crate::Options).
pub struct OptionsBuilder<'a, T: 'static> {
    name: Option<String>,
    services: &'a mut ServiceCollection,
    _marker: PhantomData<T>,
}

impl<'a, T: 'static> OptionsBuilder<'a, T> {
    /// Initializes a new options builder.
    ///
    /// # Arguments
    ///
    /// * `services` - The associated [collection of services](di::ServiceCollection)
    /// * `name` - The optional name associated with the options
    #[inline]
    pub fn new(services: &'a mut ServiceCollection, name: Option<&str>) -> Self {
        Self {
            name: name.map(|s| s.to_owned()),
            services,
            _marker: PhantomData,
        }
    }

    /// Gets the name of the options
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Gets the associated [collection of services](di::ServiceCollection)    
    #[inline]
    pub fn services(&mut self) -> &mut ServiceCollection {
        self.services
    }
}

macro_rules! builder {
    (($($bounds:tt)+)) => {
        impl<'a, T: $($bounds)+> OptionsBuilder<'a, T> {
            /// Registers an action used to configure a particular type of [`Options`](crate::Options).
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn configure<F>(self, setup: F) -> Self
            where
                F: Fn(&mut T) + $($bounds)+,
            {
                let configure = _Configure {
                    name: self.name.clone(),
                    action: setup,
                    _marker: PhantomData,
                };
                let action: Ref<dyn ConfigureOptions<T>> = Ref::new(configure);
                let descriptor = singleton_factory(move |_| action.clone());
                self.services.add(descriptor);
                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with a single dependency.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn configure1<F, D>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D>) + $($bounds)+,
                D: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure1 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency: sp.get_required::<D>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with two dependencies.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn configure2<F, D1, D2>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D1>, Svc<D2>) + $($bounds)+,
                D1: $($bounds)+,
                D2: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure2 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with three dependencies.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn configure3<F, D1, D2, D3>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D1>, Svc<D2>, Svc<D3>) + $($bounds)+,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure3 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with four dependencies.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn configure4<F, D1, D2, D3, D4>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D1>, Svc<D2>, Svc<D3>, Svc<D4>) + $($bounds)+,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
                D4: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure4 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        dependency4: sp.get_required::<D4>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with five dependencies.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn configure5<F, D1, D2, D3, D4, D5>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D1>, Svc<D2>, Svc<D3>, Svc<D4>, Svc<D5>) + $($bounds)+,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
                D4: $($bounds)+,
                D5: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure5 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        dependency4: sp.get_required::<D4>(),
                        dependency5: sp.get_required::<D5>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options).
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn post_configure<F>(self, setup: F) -> Self
            where
                F: Fn(&mut T) + $($bounds)+,
            {
                let configure = _Configure {
                    name: self.name.clone(),
                    action: setup,
                    _marker: PhantomData,
                };
                let action: Ref<dyn ConfigureOptions<T>> = Ref::new(configure);
                let descriptor = singleton_factory(move |_| action.clone());
                self.services.add(descriptor);
                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with a single dependency.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn post_configure1<F, D>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D>) + $($bounds)+,
                D: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn PostConfigureOptions<T>> = Ref::new(_Configure1 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency: sp.get_required::<D>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with two dependencies.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn post_configure2<F, D1, D2>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D1>, Svc<D2>) + $($bounds)+,
                D1: $($bounds)+,
                D2: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn PostConfigureOptions<T>> = Ref::new(_Configure2 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with three dependencies.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn post_configure3<F, D1, D2, D3>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D1>, Svc<D2>, Svc<D3>) + $($bounds)+,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn PostConfigureOptions<T>> = Ref::new(_Configure3 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with four dependencies.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn post_configure4<F, D1, D2, D3, D4>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D1>, Svc<D2>, Svc<D3>, Svc<D4>) + $($bounds)+,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
                D4: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn PostConfigureOptions<T>> = Ref::new(_Configure4 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        dependency4: sp.get_required::<D4>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to configure a particular type of [`Options`](crate::Options) with five dependencies.
            ///
            /// # Arguments
            ///
            /// * `setup` - The configuration action
            pub fn post_configure5<F, D1, D2, D3, D4, D5>(self, setup: F) -> Self
            where
                F: Fn(&mut T, Svc<D1>, Svc<D2>, Svc<D3>, Svc<D4>, Svc<D5>) + $($bounds)+,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
                D4: $($bounds)+,
                D5: $($bounds)+,
            {
                let action = Ref::new(setup);
                let name = self.name.clone();

                self.services.add(transient_factory(move |sp| {
                    let config: Ref<dyn PostConfigureOptions<T>> = Ref::new(_Configure5 {
                        name: name.clone(),
                        action: action.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        dependency4: sp.get_required::<D4>(),
                        dependency5: sp.get_required::<D5>(),
                        _marker: PhantomData,
                    });
                    config
                }));

                self
            }

            /// Registers an action used to validate a particular type of [`Options`](crate::Options).
            ///
            /// # Arguments
            ///
            /// * `action` - The validation action
            /// * `failure_message` - The message used when validation fails
            pub fn validate<F, M>(self, action: F, failure_message: M) -> Self
            where
                F: Fn(&T) -> bool + $($bounds)+,
                M: AsRef<str>,
            {
                let action = Ref::new(action);
                let name = self.name.clone();
                let failure_message = message_or_default(failure_message);

                self.services.add(transient_factory(move |_| {
                    let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate {
                        name: name.clone(),
                        action: action.clone(),
                        failure_message: failure_message.clone(),
                        _marker: PhantomData,
                    });
                    validate
                }));

                self
            }

            /// Registers an action used to validate a particular type of [`Options`](crate::Options) with a single dependency.
            ///
            /// # Arguments
            ///
            /// * `action` - The validation action
            /// * `failure_message` - The message used when validation fails
            pub fn validate1<F, M, D>(self, action: F, failure_message: M) -> Self
            where
                F: Fn(&T, Svc<D>) -> bool + $($bounds)+,
                M: AsRef<str>,
                D: $($bounds)+,
            {
                let action = Ref::new(action);
                let name = self.name.clone();
                let failure_message = message_or_default(failure_message);

                self.services.add(transient_factory(move |sp| {
                    let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate1 {
                        name: name.clone(),
                        action: action.clone(),
                        failure_message: failure_message.clone(),
                        dependency: sp.get_required::<D>(),
                        _marker: PhantomData,
                    });
                    validate
                }));

                self
            }

            /// Registers an action used to validate a particular type of [`Options`](crate::Options) with two dependencies.
            ///
            /// # Arguments
            ///
            /// * `action` - The validation action
            /// * `failure_message` - The message used when validation fails
            pub fn validate2<F, M, D1, D2>(self, action: F, failure_message: M) -> Self
            where
                F: Fn(&T, Svc<D1>, Svc<D2>) -> bool + $($bounds)+,
                M: AsRef<str>,
                D1: $($bounds)+,
                D2: $($bounds)+,
            {
                let action = Ref::new(action);
                let name = self.name.clone();
                let failure_message = message_or_default(failure_message);

                self.services.add(transient_factory(move |sp| {
                    let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate2 {
                        name: name.clone(),
                        action: action.clone(),
                        failure_message: failure_message.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        _marker: PhantomData,
                    });
                    validate
                }));

                self
            }

            /// Registers an action used to validate a particular type of [`Options`](crate::Options) with three dependencies.
            ///
            /// # Arguments
            ///
            /// * `action` - The validation action
            /// * `failure_message` - The message used when validation fails
            pub fn validate3<F, M, D1, D2, D3>(self, action: F, failure_message: M) -> Self
            where
                F: Fn(&T, Svc<D1>, Svc<D2>, Svc<D3>) -> bool + $($bounds)+,
                M: AsRef<str>,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
            {
                let action = Ref::new(action);
                let name = self.name.clone();
                let failure_message = message_or_default(failure_message);

                self.services.add(transient_factory(move |sp| {
                    let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate3 {
                        name: name.clone(),
                        action: action.clone(),
                        failure_message: failure_message.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        _marker: PhantomData,
                    });
                    validate
                }));

                self
            }

            /// Registers an action used to validate a particular type of [`Options`](crate::Options) with four dependencies.
            ///
            /// # Arguments
            ///
            /// * `action` - The validation action
            /// * `failure_message` - The message used when validation fails
            pub fn validate4<F, M, D1, D2, D3, D4>(self, action: F, failure_message: M) -> Self
            where
                F: Fn(&T, Svc<D1>, Svc<D2>, Svc<D3>, Svc<D4>) -> bool + $($bounds)+,
                M: AsRef<str>,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
                D4: $($bounds)+,
            {
                let action = Ref::new(action);
                let name = self.name.clone();
                let failure_message = message_or_default(failure_message);

                self.services.add(transient_factory(move |sp| {
                    let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate4 {
                        name: name.clone(),
                        action: action.clone(),
                        failure_message: failure_message.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        dependency4: sp.get_required::<D4>(),
                        _marker: PhantomData,
                    });
                    validate
                }));

                self
            }

            /// Registers an action used to validate a particular type of [`Options`](crate::Options) with five dependencies.
            ///
            /// # Arguments
            ///
            /// * `action` - The validation action
            /// * `failure_message` - The message used when validation fails
            pub fn validate5<F, M, D1, D2, D3, D4, D5>(self, action: F, failure_message: M) -> Self
            where
                F: Fn(&T, Svc<D1>, Svc<D2>, Svc<D3>, Svc<D4>, Svc<D5>) -> bool + $($bounds)+,
                M: AsRef<str>,
                D1: $($bounds)+,
                D2: $($bounds)+,
                D3: $($bounds)+,
                D4: $($bounds)+,
                D5: $($bounds)+,
            {
                let action = Ref::new(action);
                let name = self.name.clone();
                let failure_message = message_or_default(failure_message);

                self.services.add(transient_factory(move |sp| {
                    let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate5 {
                        name: name.clone(),
                        action: action.clone(),
                        failure_message: failure_message.clone(),
                        dependency1: sp.get_required::<D1>(),
                        dependency2: sp.get_required::<D2>(),
                        dependency3: sp.get_required::<D3>(),
                        dependency4: sp.get_required::<D4>(),
                        dependency5: sp.get_required::<D5>(),
                        _marker: PhantomData,
                    });
                    validate
                }));

                self
            }
        }
    }
}

impl<'a, T> From<OptionsBuilder<'a, T>> for &'a mut ServiceCollection {
    #[inline]
    fn from(builder: OptionsBuilder<'a, T>) -> Self {
        builder.services
    }
}

impl<'a, T> Deref for OptionsBuilder<'a, T> {
    type Target = ServiceCollection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.services
    }
}

impl<'a, T> DerefMut for OptionsBuilder<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.services
    }
}

fn names_equal(name: Option<&str>, other_name: Option<&str>) -> bool {
    let matches_all = name.is_none();

    if matches_all || name == other_name {
        return true;
    } else if other_name.is_none() {
        return false;
    }

    let name1 = name.unwrap();
    let name2 = other_name.unwrap();

    (name1.len() == name2.len())
        && ((name1.to_uppercase() == name2.to_uppercase()) || (name1.to_lowercase() == name2.to_lowercase()))
}

fn message_or_default<T: AsRef<str>>(message: T) -> String {
    let msg = message.as_ref();

    if msg.is_empty() {
        String::from("A validation error has occurred.")
    } else {
        String::from(msg)
    }
}

macro_rules! builder_impl {
    (($($bounds:tt)+)) => {
        struct _Configure<TOptions, TAction> {
            name: Option<String>,
            action: TAction,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction> ConfigureOptions<TOptions> for _Configure<TOptions, TAction>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions) + $($bounds)+,
        {
            fn configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(options)
                }
            }
        }

        impl<TOptions, TAction> PostConfigureOptions<TOptions> for _Configure<TOptions, TAction>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions) + $($bounds)+,
        {
            fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(options)
                }
            }
        }

        struct _Configure1<TOptions, TAction, TDep> {
            name: Option<String>,
            action: Ref<TAction>,
            dependency: Svc<TDep>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep> ConfigureOptions<TOptions> for _Configure1<TOptions, TAction, TDep>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep>) + $($bounds)+,
            TDep: Value,
        {
            fn configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(options, self.dependency.clone())
                }
            }
        }

        impl<TOptions, TAction, TDep> PostConfigureOptions<TOptions>
            for _Configure1<TOptions, TAction, TDep>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep>) + $($bounds)+,
            TDep: Value,
        {
            fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(options, self.dependency.clone())
                }
            }
        }

        struct _Configure2<TOptions, TAction, TDep1, TDep2> {
            name: Option<String>,
            action: Ref<TAction>,
            dependency1: Svc<TDep1>,
            dependency2: Svc<TDep2>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep1, TDep2> ConfigureOptions<TOptions>
            for _Configure2<TOptions, TAction, TDep1, TDep2>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep1>, Svc<TDep2>) + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
        {
            fn configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(options, self.dependency1.clone(), self.dependency2.clone())
                }
            }
        }

        impl<TOptions, TAction, TDep1, TDep2> PostConfigureOptions<TOptions>
            for _Configure2<TOptions, TAction, TDep1, TDep2>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep1>, Svc<TDep2>) + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
        {
            fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(options, self.dependency1.clone(), self.dependency2.clone())
                }
            }
        }

        struct _Configure3<TOptions, TAction, TDep1, TDep2, TDep3> {
            name: Option<String>,
            action: Ref<TAction>,
            dependency1: Svc<TDep1>,
            dependency2: Svc<TDep2>,
            dependency3: Svc<TDep3>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3> ConfigureOptions<TOptions>
            for _Configure3<TOptions, TAction, TDep1, TDep2, TDep3>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep1>, Svc<TDep2>, Svc<TDep3>) + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
        {
            fn configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                    )
                }
            }
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3> PostConfigureOptions<TOptions>
            for _Configure3<TOptions, TAction, TDep1, TDep2, TDep3>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep1>, Svc<TDep2>, Svc<TDep3>) + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
        {
            fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                    )
                }
            }
        }

        struct _Configure4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4> {
            name: Option<String>,
            action: Ref<TAction>,
            dependency1: Svc<TDep1>,
            dependency2: Svc<TDep2>,
            dependency3: Svc<TDep3>,
            dependency4: Svc<TDep4>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4> ConfigureOptions<TOptions>
            for _Configure4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep1>, Svc<TDep2>, Svc<TDep3>, Svc<TDep4>) + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
            TDep4: Value,
        {
            fn configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                        self.dependency4.clone(),
                    )
                }
            }
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4> PostConfigureOptions<TOptions>
            for _Configure4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep1>, Svc<TDep2>, Svc<TDep3>, Svc<TDep4>) + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
            TDep4: Value,
        {
            fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                        self.dependency4.clone(),
                    )
                }
            }
        }

        struct _Configure5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5> {
            name: Option<String>,
            action: Ref<TAction>,
            dependency1: Svc<TDep1>,
            dependency2: Svc<TDep2>,
            dependency3: Svc<TDep3>,
            dependency4: Svc<TDep4>,
            dependency5: Svc<TDep5>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5> ConfigureOptions<TOptions>
            for _Configure5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep1>, Svc<TDep2>, Svc<TDep3>, Svc<TDep4>, Svc<TDep5>) + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
            TDep4: Value,
            TDep5: Value,
        {
            fn configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                        self.dependency4.clone(),
                        self.dependency5.clone(),
                    )
                }
            }
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5> PostConfigureOptions<TOptions>
            for _Configure5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, Svc<TDep1>, Svc<TDep2>, Svc<TDep3>, Svc<TDep4>, Svc<TDep5>) + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
            TDep4: Value,
            TDep5: Value,
        {
            fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
                if names_equal(self.name.as_deref(), name) {
                    (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                        self.dependency4.clone(),
                        self.dependency5.clone(),
                    )
                }
            }
        }

        struct _Validate<TOptions, TAction> {
            name: Option<String>,
            action: Ref<TAction>,
            failure_message: String,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction> ValidateOptions<TOptions> for _Validate<TOptions, TAction>
        where
            TOptions: Value,
            TAction: Fn(&TOptions) -> bool + $($bounds)+,
        {
            fn validate(&self, name: Option<&str>, options: &TOptions) -> ValidateOptionsResult {
                if names_equal(self.name.as_deref(), name) {
                    if (self.action)(options) {
                        return ValidateOptionsResult::success();
                    } else {
                        return ValidateOptionsResult::fail(&self.failure_message);
                    }
                }

                return ValidateOptionsResult::skip();
            }
        }

        struct _Validate1<TOptions, TAction, TDep> {
            name: Option<String>,
            action: Ref<TAction>,
            failure_message: String,
            dependency: Svc<TDep>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep> ValidateOptions<TOptions> for _Validate1<TOptions, TAction, TDep>
        where
            TOptions: Value,
            TAction: Fn(&TOptions, Svc<TDep>) -> bool + $($bounds)+,
            TDep: Value,
        {
            fn validate(&self, name: Option<&str>, options: &TOptions) -> ValidateOptionsResult {
                if names_equal(self.name.as_deref(), name) {
                    if (self.action)(options, self.dependency.clone()) {
                        return ValidateOptionsResult::success();
                    } else {
                        return ValidateOptionsResult::fail(&self.failure_message);
                    }
                }

                return ValidateOptionsResult::skip();
            }
        }

        struct _Validate2<TOptions, TAction, TDep1, TDep2> {
            name: Option<String>,
            action: Ref<TAction>,
            failure_message: String,
            dependency1: Svc<TDep1>,
            dependency2: Svc<TDep2>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep1, TDep2> ValidateOptions<TOptions>
            for _Validate2<TOptions, TAction, TDep1, TDep2>
        where
            TOptions: Value,
            TAction: Fn(&TOptions, Svc<TDep1>, Svc<TDep2>) -> bool + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
        {
            fn validate(&self, name: Option<&str>, options: &TOptions) -> ValidateOptionsResult {
                if names_equal(self.name.as_deref(), name) {
                    if (self.action)(options, self.dependency1.clone(), self.dependency2.clone()) {
                        return ValidateOptionsResult::success();
                    } else {
                        return ValidateOptionsResult::fail(&self.failure_message);
                    }
                }

                return ValidateOptionsResult::skip();
            }
        }

        struct _Validate3<TOptions, TAction, TDep1, TDep2, TDep3> {
            name: Option<String>,
            action: Ref<TAction>,
            failure_message: String,
            dependency1: Svc<TDep1>,
            dependency2: Svc<TDep2>,
            dependency3: Svc<TDep3>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3> ValidateOptions<TOptions>
            for _Validate3<TOptions, TAction, TDep1, TDep2, TDep3>
        where
            TOptions: Value,
            TAction: Fn(&TOptions, Svc<TDep1>, Svc<TDep2>, Svc<TDep3>) -> bool + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
        {
            fn validate(&self, name: Option<&str>, options: &TOptions) -> ValidateOptionsResult {
                if names_equal(self.name.as_deref(), name) {
                    if (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                    ) {
                        return ValidateOptionsResult::success();
                    } else {
                        return ValidateOptionsResult::fail(&self.failure_message);
                    }
                }

                return ValidateOptionsResult::skip();
            }
        }

        struct _Validate4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4> {
            name: Option<String>,
            action: Ref<TAction>,
            failure_message: String,
            dependency1: Svc<TDep1>,
            dependency2: Svc<TDep2>,
            dependency3: Svc<TDep3>,
            dependency4: Svc<TDep4>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4> ValidateOptions<TOptions>
            for _Validate4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
        where
            TOptions: Value,
            TAction: Fn(&TOptions, Ref<TDep1>, Ref<TDep2>, Ref<TDep3>, Ref<TDep4>) -> bool + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
            TDep4: Value,
        {
            fn validate(&self, name: Option<&str>, options: &TOptions) -> ValidateOptionsResult {
                if names_equal(self.name.as_deref(), name) {
                    if (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                        self.dependency4.clone(),
                    ) {
                        return ValidateOptionsResult::success();
                    } else {
                        return ValidateOptionsResult::fail(&self.failure_message);
                    }
                }

                return ValidateOptionsResult::skip();
            }
        }

        struct _Validate5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5> {
            name: Option<String>,
            action: Ref<TAction>,
            failure_message: String,
            dependency1: Svc<TDep1>,
            dependency2: Svc<TDep2>,
            dependency3: Svc<TDep3>,
            dependency4: Svc<TDep4>,
            dependency5: Svc<TDep5>,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5> ValidateOptions<TOptions>
            for _Validate5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
        where
            TOptions: Value,
            TAction: Fn(&TOptions, Svc<TDep1>, Svc<TDep2>, Svc<TDep3>, Svc<TDep4>, Svc<TDep5>) -> bool + $($bounds)+,
            TDep1: Value,
            TDep2: Value,
            TDep3: Value,
            TDep4: Value,
            TDep5: Value,
        {
            fn validate(&self, name: Option<&str>, options: &TOptions) -> ValidateOptionsResult {
                if names_equal(self.name.as_deref(), name) {
                    if (self.action)(
                        options,
                        self.dependency1.clone(),
                        self.dependency2.clone(),
                        self.dependency3.clone(),
                        self.dependency4.clone(),
                        self.dependency5.clone(),
                    ) {
                        return ValidateOptionsResult::success();
                    } else {
                        return ValidateOptionsResult::fail(&self.failure_message);
                    }
                }

                return ValidateOptionsResult::skip();
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "async")] {
        builder!((Send + Sync + 'static));
        builder_impl!((Send + Sync + 'static));
    } else {
        builder!(('static));
        builder_impl!(('static));
    }
}

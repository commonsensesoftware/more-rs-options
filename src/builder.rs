use crate::{ConfigureOptions, PostConfigureOptions, ValidateOptions, ValidateOptionsResult};
use di::{singleton_factory, transient_factory, ServiceCollection, Ref};
use std::ops::{Deref, DerefMut};
use std::{marker::PhantomData, rc::Rc};

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
    pub fn new(services: &'a mut ServiceCollection, name: Option<&str>) -> Self {
        Self {
            name: name.map(|s| s.to_owned()),
            services,
            _marker: PhantomData,
        }
    }

    /// Gets the name of the options
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Gets the associated [collection of services](di::ServiceCollection)
    pub fn services(&mut self) -> &mut ServiceCollection {
        self.services
    }

    /// Registers an action used to configure a particular type of [`Options`](crate::Options).
    ///
    /// # Arguments
    ///
    /// * `setup` - The configuration action
    pub fn configure<F>(self, setup: F) -> Self
    where
        F: Fn(&mut T) + 'static,
    {
        let configure = _Configure::new(self.name.clone(), setup);
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
        F: Fn(&mut T, Ref<D>) + 'static,
        D: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure1::new(
                name.clone(),
                sp.get_required::<D>(),
                action.clone(),
            ));
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
        F: Fn(&mut T, Ref<D1>, Ref<D2>) + 'static,
        D1: 'static,
        D2: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure2::new(
                name.clone(),
                sp.get_required::<D1>(),
                sp.get_required::<D2>(),
                action.clone(),
            ));
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
        F: Fn(&mut T, Ref<D1>, Ref<D2>, Ref<D3>) + 'static,
        D1: 'static,
        D2: 'static,
        D3: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure3::new(
                name.clone(),
                sp.get_required::<D1>(),
                sp.get_required::<D2>(),
                sp.get_required::<D3>(),
                action.clone(),
            ));
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
        F: Fn(&mut T, Ref<D1>, Ref<D2>, Ref<D3>, Ref<D4>) + 'static,
        D1: 'static,
        D2: 'static,
        D3: 'static,
        D4: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure4::new(
                name.clone(),
                sp.get_required::<D1>(),
                sp.get_required::<D2>(),
                sp.get_required::<D3>(),
                sp.get_required::<D4>(),
                action.clone(),
            ));
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
        F: Fn(
                &mut T,
                Ref<D1>,
                Ref<D2>,
                Ref<D3>,
                Ref<D4>,
                Ref<D5>,
            ) + 'static,
        D1: 'static,
        D2: 'static,
        D3: 'static,
        D4: 'static,
        D5: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn ConfigureOptions<T>> = Ref::new(_Configure5::new(
                name.clone(),
                sp.get_required::<D1>(),
                sp.get_required::<D2>(),
                sp.get_required::<D3>(),
                sp.get_required::<D4>(),
                sp.get_required::<D5>(),
                action.clone(),
            ));
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
        F: Fn(&mut T) + 'static,
    {
        let configure = _Configure::new(self.name.clone(), setup);
        let action: Ref<dyn PostConfigureOptions<T>> = Ref::new(configure);
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
        F: Fn(&mut T, Ref<D>) + 'static,
        D: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn PostConfigureOptions<T>> = Ref::new(
                _Configure1::new(name.clone(), sp.get_required::<D>(), action.clone()),
            );
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
        F: Fn(&mut T, Ref<D1>, Ref<D2>) + 'static,
        D1: 'static,
        D2: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn PostConfigureOptions<T>> =
                Ref::new(_Configure2::new(
                    name.clone(),
                    sp.get_required::<D1>(),
                    sp.get_required::<D2>(),
                    action.clone(),
                ));
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
        F: Fn(&mut T, Ref<D1>, Ref<D2>, Ref<D3>) + 'static,
        D1: 'static,
        D2: 'static,
        D3: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn PostConfigureOptions<T>> =
                Ref::new(_Configure3::new(
                    name.clone(),
                    sp.get_required::<D1>(),
                    sp.get_required::<D2>(),
                    sp.get_required::<D3>(),
                    action.clone(),
                ));
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
        F: Fn(&mut T, Ref<D1>, Ref<D2>, Ref<D3>, Ref<D4>) + 'static,
        D1: 'static,
        D2: 'static,
        D3: 'static,
        D4: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn PostConfigureOptions<T>> =
                Ref::new(_Configure4::new(
                    name.clone(),
                    sp.get_required::<D1>(),
                    sp.get_required::<D2>(),
                    sp.get_required::<D3>(),
                    sp.get_required::<D4>(),
                    action.clone(),
                ));
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
        F: Fn(
                &mut T,
                Ref<D1>,
                Ref<D2>,
                Ref<D3>,
                Ref<D4>,
                Ref<D5>,
            ) + 'static,
        D1: 'static,
        D2: 'static,
        D3: 'static,
        D4: 'static,
        D5: 'static,
    {
        let action = Rc::new(setup);
        let name = self.name.clone();

        self.services.add(transient_factory(move |sp| {
            let config: Ref<dyn PostConfigureOptions<T>> =
                Ref::new(_Configure5::new(
                    name.clone(),
                    sp.get_required::<D1>(),
                    sp.get_required::<D2>(),
                    sp.get_required::<D3>(),
                    sp.get_required::<D4>(),
                    sp.get_required::<D5>(),
                    action.clone(),
                ));
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
        F: Fn(&T) -> bool + 'static,
        M: AsRef<str>,
    {
        let validate = _Validate::new(
            self.name.clone(),
            message_or_default(failure_message),
            action,
        );
        let action: Ref<dyn ValidateOptions<T>> = Ref::new(validate);
        let descriptor = transient_factory(move |_| action.clone());
        self.services.add(descriptor);
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
        F: Fn(&T, Ref<D>) -> bool + 'static,
        M: AsRef<str>,
        D: 'static,
    {
        let action = Rc::new(action);
        let name = self.name.clone();
        let failure_message = message_or_default(failure_message);

        self.services.add(transient_factory(move |sp| {
            let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate1::new(
                name.clone(),
                failure_message.clone(),
                sp.get_required::<D>(),
                action.clone(),
            ));
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
        F: Fn(&T, Ref<D1>, Ref<D2>) -> bool + 'static,
        M: AsRef<str>,
        D1: 'static,
        D2: 'static,
    {
        let action = Rc::new(action);
        let name = self.name.clone();
        let failure_message = message_or_default(failure_message);

        self.services.add(transient_factory(move |sp| {
            let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate2::new(
                name.clone(),
                failure_message.clone(),
                sp.get_required::<D1>(),
                sp.get_required::<D2>(),
                action.clone(),
            ));
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
        F: Fn(&T, Ref<D1>, Ref<D2>, Ref<D3>) -> bool + 'static,
        M: AsRef<str>,
        D1: 'static,
        D2: 'static,
        D3: 'static,
    {
        let action = Rc::new(action);
        let name = self.name.clone();
        let failure_message = message_or_default(failure_message);

        self.services.add(transient_factory(move |sp| {
            let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate3::new(
                name.clone(),
                failure_message.clone(),
                sp.get_required::<D1>(),
                sp.get_required::<D2>(),
                sp.get_required::<D3>(),
                action.clone(),
            ));
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
        F: Fn(&T, Ref<D1>, Ref<D2>, Ref<D3>, Ref<D4>) -> bool + 'static,
        M: AsRef<str>,
        D1: 'static,
        D2: 'static,
        D3: 'static,
        D4: 'static,
    {
        let action = Rc::new(action);
        let name = self.name.clone();
        let failure_message = message_or_default(failure_message);

        self.services.add(transient_factory(move |sp| {
            let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate4::new(
                name.clone(),
                failure_message.clone(),
                sp.get_required::<D1>(),
                sp.get_required::<D2>(),
                sp.get_required::<D3>(),
                sp.get_required::<D4>(),
                action.clone(),
            ));
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
        F: Fn(
                &T,
                Ref<D1>,
                Ref<D2>,
                Ref<D3>,
                Ref<D4>,
                Ref<D5>,
            ) -> bool
            + 'static,
        M: AsRef<str>,
        D1: 'static,
        D2: 'static,
        D3: 'static,
        D4: 'static,
        D5: 'static,
    {
        let action = Rc::new(action);
        let name = self.name.clone();
        let failure_message = message_or_default(failure_message);

        self.services.add(transient_factory(move |sp| {
            let validate: Ref<dyn ValidateOptions<T>> = Ref::new(_Validate5::new(
                name.clone(),
                failure_message.clone(),
                sp.get_required::<D1>(),
                sp.get_required::<D2>(),
                sp.get_required::<D3>(),
                sp.get_required::<D4>(),
                sp.get_required::<D5>(),
                action.clone(),
            ));
            validate
        }));

        self
    }
}

fn names_equal(name: Option<&str>, other_name: Option<&str>) -> bool {
    let matches_all = name.is_none();

    if matches_all || name == other_name {
        return true;
    }

    let name1 = name.clone().unwrap();
    let name2 = other_name.clone().unwrap();

    (name1.len() == name2.len())
        && ((name1.to_uppercase() == name2.to_uppercase())
            || (name1.to_lowercase() == name2.to_lowercase()))
}

impl<'a, T> Into<&'a mut ServiceCollection> for OptionsBuilder<'a, T> {
    fn into(self) -> &'a mut ServiceCollection {
        self.services
    }
}

impl<'a, T> Deref for OptionsBuilder<'a, T> {
    type Target = ServiceCollection;

    fn deref(&self) -> &Self::Target {
        self.services
    }
}

impl<'a, T> DerefMut for OptionsBuilder<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.services
    }
}

struct _Configure<TOptions, TAction>
where
    TAction: Fn(&mut TOptions),
{
    name: Option<String>,
    action: TAction,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction> _Configure<TOptions, TAction>
where
    TAction: Fn(&mut TOptions),
{
    fn new(name: Option<String>, action: TAction) -> Self {
        Self {
            name,
            action,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction> ConfigureOptions<TOptions> for _Configure<TOptions, TAction>
where
    TAction: Fn(&mut TOptions),
{
    fn configure(&self, name: Option<&str>, options: &mut TOptions) {
        if names_equal(self.name.as_deref(), name) {
            (self.action)(options)
        }
    }
}

impl<TOptions, TAction> PostConfigureOptions<TOptions> for _Configure<TOptions, TAction>
where
    TAction: Fn(&mut TOptions),
{
    fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
        if names_equal(self.name.as_deref(), name) {
            (self.action)(options)
        }
    }
}

struct _Configure1<TOptions, TAction, TDep>
where
    TAction: Fn(&mut TOptions, Ref<TDep>),
{
    name: Option<String>,
    action: Rc<TAction>,
    dependency: Ref<TDep>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep> _Configure1<TOptions, TAction, TDep>
where
    TAction: Fn(&mut TOptions, Ref<TDep>),
{
    fn new(name: Option<String>, dependency: Ref<TDep>, action: Rc<TAction>) -> Self {
        Self {
            name,
            action,
            dependency,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep> ConfigureOptions<TOptions> for _Configure1<TOptions, TAction, TDep>
where
    TAction: Fn(&mut TOptions, Ref<TDep>),
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
    TAction: Fn(&mut TOptions, Ref<TDep>),
{
    fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
        if names_equal(self.name.as_deref(), name) {
            (self.action)(options, self.dependency.clone())
        }
    }
}

struct _Configure2<TOptions, TAction, TDep1, TDep2>
where
    TAction: Fn(&mut TOptions, Ref<TDep1>, Ref<TDep2>),
{
    name: Option<String>,
    action: Rc<TAction>,
    dependency1: Ref<TDep1>,
    dependency2: Ref<TDep2>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep1, TDep2> _Configure2<TOptions, TAction, TDep1, TDep2>
where
    TAction: Fn(&mut TOptions, Ref<TDep1>, Ref<TDep2>),
{
    fn new(
        name: Option<String>,
        dependency1: Ref<TDep1>,
        dependency2: Ref<TDep2>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            action,
            dependency1,
            dependency2,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep1, TDep2> ConfigureOptions<TOptions>
    for _Configure2<TOptions, TAction, TDep1, TDep2>
where
    TAction: Fn(&mut TOptions, Ref<TDep1>, Ref<TDep2>),
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
    TAction: Fn(&mut TOptions, Ref<TDep1>, Ref<TDep2>),
{
    fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
        if names_equal(self.name.as_deref(), name) {
            (self.action)(options, self.dependency1.clone(), self.dependency2.clone())
        }
    }
}

struct _Configure3<TOptions, TAction, TDep1, TDep2, TDep3>
where
    TAction: Fn(&mut TOptions, Ref<TDep1>, Ref<TDep2>, Ref<TDep3>),
{
    name: Option<String>,
    action: Rc<TAction>,
    dependency1: Ref<TDep1>,
    dependency2: Ref<TDep2>,
    dependency3: Ref<TDep3>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep1, TDep2, TDep3> _Configure3<TOptions, TAction, TDep1, TDep2, TDep3>
where
    TAction: Fn(&mut TOptions, Ref<TDep1>, Ref<TDep2>, Ref<TDep3>),
{
    fn new(
        name: Option<String>,
        dependency1: Ref<TDep1>,
        dependency2: Ref<TDep2>,
        dependency3: Ref<TDep3>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            action,
            dependency1,
            dependency2,
            dependency3,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep1, TDep2, TDep3> ConfigureOptions<TOptions>
    for _Configure3<TOptions, TAction, TDep1, TDep2, TDep3>
where
    TAction: Fn(&mut TOptions, Ref<TDep1>, Ref<TDep2>, Ref<TDep3>),
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
    TAction: Fn(&mut TOptions, Ref<TDep1>, Ref<TDep2>, Ref<TDep3>),
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

struct _Configure4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
where
    TAction: Fn(
        &mut TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
    ),
{
    name: Option<String>,
    action: Rc<TAction>,
    dependency1: Ref<TDep1>,
    dependency2: Ref<TDep2>,
    dependency3: Ref<TDep3>,
    dependency4: Ref<TDep4>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
    _Configure4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
where
    TAction: Fn(
        &mut TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
    ),
{
    fn new(
        name: Option<String>,
        dependency1: Ref<TDep1>,
        dependency2: Ref<TDep2>,
        dependency3: Ref<TDep3>,
        dependency4: Ref<TDep4>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            action,
            dependency1,
            dependency2,
            dependency3,
            dependency4,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4> ConfigureOptions<TOptions>
    for _Configure4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
where
    TAction: Fn(
        &mut TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
    ),
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
    TAction: Fn(
        &mut TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
    ),
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

struct _Configure5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
where
    TAction: Fn(
        &mut TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
        Ref<TDep5>,
    ),
{
    name: Option<String>,
    action: Rc<TAction>,
    dependency1: Ref<TDep1>,
    dependency2: Ref<TDep2>,
    dependency3: Ref<TDep3>,
    dependency4: Ref<TDep4>,
    dependency5: Ref<TDep5>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
    _Configure5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
where
    TAction: Fn(
        &mut TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
        Ref<TDep5>,
    ),
{
    fn new(
        name: Option<String>,
        dependency1: Ref<TDep1>,
        dependency2: Ref<TDep2>,
        dependency3: Ref<TDep3>,
        dependency4: Ref<TDep4>,
        dependency5: Ref<TDep5>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            action,
            dependency1,
            dependency2,
            dependency3,
            dependency4,
            dependency5,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5> ConfigureOptions<TOptions>
    for _Configure5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
where
    TAction: Fn(
        &mut TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
        Ref<TDep5>,
    ),
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
    TAction: Fn(
        &mut TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
        Ref<TDep5>,
    ),
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

fn message_or_default<T: AsRef<str>>(message: T) -> String {
    let msg = message.as_ref();

    if msg.is_empty() {
        String::from("A validation error has occurred.")
    } else {
        String::from(msg)
    }
}

struct _Validate<TOptions, TAction>
where
    TAction: Fn(&TOptions) -> bool,
{
    name: Option<String>,
    failure_message: String,
    action: TAction,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction> _Validate<TOptions, TAction>
where
    TAction: Fn(&TOptions) -> bool,
{
    fn new(name: Option<String>, failure_message: String, action: TAction) -> Self {
        Self {
            name,
            failure_message,
            action,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction> ValidateOptions<TOptions> for _Validate<TOptions, TAction>
where
    TAction: Fn(&TOptions) -> bool,
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

struct _Validate1<TOptions, TAction, TDep>
where
    TAction: Fn(&TOptions, Ref<TDep>) -> bool,
{
    name: Option<String>,
    failure_message: String,
    action: Rc<TAction>,
    dependency1: Ref<TDep>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep> _Validate1<TOptions, TAction, TDep>
where
    TAction: Fn(&TOptions, Ref<TDep>) -> bool,
{
    fn new(
        name: Option<String>,
        failure_message: String,
        dependency1: Ref<TDep>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            failure_message,
            action,
            dependency1,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep> ValidateOptions<TOptions> for _Validate1<TOptions, TAction, TDep>
where
    TAction: Fn(&TOptions, Ref<TDep>) -> bool,
{
    fn validate(&self, name: Option<&str>, options: &TOptions) -> ValidateOptionsResult {
        if names_equal(self.name.as_deref(), name) {
            if (self.action)(options, self.dependency1.clone()) {
                return ValidateOptionsResult::success();
            } else {
                return ValidateOptionsResult::fail(&self.failure_message);
            }
        }

        return ValidateOptionsResult::skip();
    }
}

struct _Validate2<TOptions, TAction, TDep1, TDep2>
where
    TAction: Fn(&TOptions, Ref<TDep1>, Ref<TDep2>) -> bool,
{
    name: Option<String>,
    failure_message: String,
    action: Rc<TAction>,
    dependency1: Ref<TDep1>,
    dependency2: Ref<TDep2>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep1, TDep2> _Validate2<TOptions, TAction, TDep1, TDep2>
where
    TAction: Fn(&TOptions, Ref<TDep1>, Ref<TDep2>) -> bool,
{
    fn new(
        name: Option<String>,
        failure_message: String,
        dependency1: Ref<TDep1>,
        dependency2: Ref<TDep2>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            failure_message,
            action,
            dependency1,
            dependency2,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep1, TDep2> ValidateOptions<TOptions>
    for _Validate2<TOptions, TAction, TDep1, TDep2>
where
    TAction: Fn(&TOptions, Ref<TDep1>, Ref<TDep2>) -> bool,
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

struct _Validate3<TOptions, TAction, TDep1, TDep2, TDep3>
where
    TAction: Fn(&TOptions, Ref<TDep1>, Ref<TDep2>, Ref<TDep3>) -> bool,
{
    name: Option<String>,
    failure_message: String,
    action: Rc<TAction>,
    dependency1: Ref<TDep1>,
    dependency2: Ref<TDep2>,
    dependency3: Ref<TDep3>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep1, TDep2, TDep3> _Validate3<TOptions, TAction, TDep1, TDep2, TDep3>
where
    TAction: Fn(&TOptions, Ref<TDep1>, Ref<TDep2>, Ref<TDep3>) -> bool,
{
    fn new(
        name: Option<String>,
        failure_message: String,
        dependency1: Ref<TDep1>,
        dependency2: Ref<TDep2>,
        dependency3: Ref<TDep3>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            failure_message,
            action,
            dependency1,
            dependency2,
            dependency3,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep1, TDep2, TDep3> ValidateOptions<TOptions>
    for _Validate3<TOptions, TAction, TDep1, TDep2, TDep3>
where
    TAction: Fn(&TOptions, Ref<TDep1>, Ref<TDep2>, Ref<TDep3>) -> bool,
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

struct _Validate4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
where
    TAction: Fn(
        &TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
    ) -> bool,
{
    name: Option<String>,
    failure_message: String,
    action: Rc<TAction>,
    dependency1: Ref<TDep1>,
    dependency2: Ref<TDep2>,
    dependency3: Ref<TDep3>,
    dependency4: Ref<TDep4>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
    _Validate4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
where
    TAction: Fn(
        &TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
    ) -> bool,
{
    fn new(
        name: Option<String>,
        failure_message: String,
        dependency1: Ref<TDep1>,
        dependency2: Ref<TDep2>,
        dependency3: Ref<TDep3>,
        dependency4: Ref<TDep4>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            failure_message,
            action,
            dependency1,
            dependency2,
            dependency3,
            dependency4,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4> ValidateOptions<TOptions>
    for _Validate4<TOptions, TAction, TDep1, TDep2, TDep3, TDep4>
where
    TAction: Fn(
        &TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
    ) -> bool,
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

struct _Validate5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
where
    TAction: Fn(
        &TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
        Ref<TDep5>,
    ) -> bool,
{
    name: Option<String>,
    failure_message: String,
    action: Rc<TAction>,
    dependency1: Ref<TDep1>,
    dependency2: Ref<TDep2>,
    dependency3: Ref<TDep3>,
    dependency4: Ref<TDep4>,
    dependency5: Ref<TDep5>,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
    _Validate5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
where
    TAction: Fn(
        &TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
        Ref<TDep5>,
    ) -> bool,
{
    fn new(
        name: Option<String>,
        failure_message: String,
        dependency1: Ref<TDep1>,
        dependency2: Ref<TDep2>,
        dependency3: Ref<TDep3>,
        dependency4: Ref<TDep4>,
        dependency5: Ref<TDep5>,
        action: Rc<TAction>,
    ) -> Self {
        Self {
            name,
            failure_message,
            action,
            dependency1,
            dependency2,
            dependency3,
            dependency4,
            dependency5,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5> ValidateOptions<TOptions>
    for _Validate5<TOptions, TAction, TDep1, TDep2, TDep3, TDep4, TDep5>
where
    TAction: Fn(
        &TOptions,
        Ref<TDep1>,
        Ref<TDep2>,
        Ref<TDep3>,
        Ref<TDep4>,
        Ref<TDep5>,
    ) -> bool,
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

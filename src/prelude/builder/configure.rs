use super::{names_equal, Builder};
use crate::{Configure, PostConfigure, Ref, Value};
use cfg_if::cfg_if;
use di::{singleton_factory, transient_factory, Ref as Svc};
use std::marker::PhantomData;

macro_rules! configure_impl {
    // base case: zero dependencies
    (($($bounds:tt)+), _Configure, []) => {
        struct _Configure<TOptions, TAction> {
            name: String,
            action: TAction,
            _type: PhantomData<TOptions>,
        }

        impl<TOptions, TAction> Configure<TOptions> for _Configure<TOptions, TAction>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions) + $($bounds)+,
        {
            fn run(&self, name: &str, options: &mut TOptions) {
                if names_equal(&self.name, name) {
                    (self.action)(options)
                }
            }
        }

        impl<TOptions, TAction> PostConfigure<TOptions> for _Configure<TOptions, TAction>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions) + $($bounds)+,
        {
            fn run(&self, name: &str, options: &mut TOptions) {
                if names_equal(&self.name, name) {
                    (self.action)(options)
                }
            }
        }
    };
    // n-dependency case
    (($($bounds:tt)+), $struct_name:ident, [$(($dep_generic:ident, $dep_field:ident)),+]) => {
        struct $struct_name<TOptions, TAction, $($dep_generic),+> {
            name: String,
            action: Ref<TAction>,
            $($dep_field: Svc<$dep_generic>,)+
            _type: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, $($dep_generic),+> Configure<TOptions>
            for $struct_name<TOptions, TAction, $($dep_generic),+>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, $(Svc<$dep_generic>),+) + $($bounds)+,
            $($dep_generic: Value,)+
        {
            fn run(&self, name: &str, options: &mut TOptions) {
                if names_equal(&self.name, name) {
                    (self.action)(options, $(self.$dep_field.clone()),+)
                }
            }
        }

        impl<TOptions, TAction, $($dep_generic),+> PostConfigure<TOptions>
            for $struct_name<TOptions, TAction, $($dep_generic),+>
        where
            TOptions: Value,
            TAction: Fn(&mut TOptions, $(Svc<$dep_generic>),+) + $($bounds)+,
            $($dep_generic: Value,)+
        {
            fn run(&self, name: &str, options: &mut TOptions) {
                if names_equal(&self.name, name) {
                    (self.action)(options, $(self.$dep_field.clone()),+)
                }
            }
        }
    };
}

// generates builder methods for configure/post_configure with n-dependencies
macro_rules! configure_builder_method {
    (($($bounds:tt)+), $method:ident, $post_method:ident, $struct_name:ident, ($d1:ident, $f1:ident)) => {
        /// Registers an action used to configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $method<F, $d1>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>) + $($bounds)+,
            $d1: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn Configure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }

        /// Registers an action used to post-configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $post_method<F, $d1>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>) + $($bounds)+,
            $d1: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn PostConfigure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }
    };
    (($($bounds:tt)+), $method:ident, $post_method:ident, $struct_name:ident, ($d1:ident, $f1:ident), ($d2:ident, $f2:ident)) => {
        /// Registers an action used to configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $method<F, $d1, $d2>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>, Svc<$d2>) + $($bounds)+,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn Configure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }

        /// Registers an action used to post-configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $post_method<F, $d1, $d2>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>, Svc<$d2>) + $($bounds)+,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn PostConfigure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }
    };
    (($($bounds:tt)+), $method:ident, $post_method:ident, $struct_name:ident, ($d1:ident, $f1:ident), ($d2:ident, $f2:ident), ($d3:ident, $f3:ident)) => {
        /// Registers an action used to configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $method<F, $d1, $d2, $d3>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>, Svc<$d2>, Svc<$d3>) + $($bounds)+,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn Configure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }

        /// Registers an action used to post-configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $post_method<F, $d1, $d2, $d3>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>, Svc<$d2>, Svc<$d3>) + $($bounds)+,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn PostConfigure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }
    };
    (($($bounds:tt)+), $method:ident, $post_method:ident, $struct_name:ident, ($d1:ident, $f1:ident), ($d2:ident, $f2:ident), ($d3:ident, $f3:ident), ($d4:ident, $f4:ident)) => {
        /// Registers an action used to configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $method<F, $d1, $d2, $d3, $d4>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>, Svc<$d2>, Svc<$d3>, Svc<$d4>) + $($bounds)+,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
            $d4: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn Configure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    $f4: sp.get_required::<$d4>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }

        /// Registers an action used to post-configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $post_method<F, $d1, $d2, $d3, $d4>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>, Svc<$d2>, Svc<$d3>, Svc<$d4>) + $($bounds)+,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
            $d4: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn PostConfigure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    $f4: sp.get_required::<$d4>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }
    };
    (($($bounds:tt)+), $method:ident, $post_method:ident, $struct_name:ident, ($d1:ident, $f1:ident), ($d2:ident, $f2:ident), ($d3:ident, $f3:ident), ($d4:ident, $f4:ident), ($d5:ident, $f5:ident)) => {
        /// Registers an action used to configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $method<F, $d1, $d2, $d3, $d4, $d5>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>, Svc<$d2>, Svc<$d3>, Svc<$d4>, Svc<$d5>) + $($bounds)+,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
            $d4: $($bounds)+,
            $d5: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn Configure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    $f4: sp.get_required::<$d4>(),
                    $f5: sp.get_required::<$d5>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }

        /// Registers an action used to post-configure a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `setup` - The configuration action
        pub fn $post_method<F, $d1, $d2, $d3, $d4, $d5>(self, setup: F) -> Self
        where
            F: Fn(&mut T, Svc<$d1>, Svc<$d2>, Svc<$d3>, Svc<$d4>, Svc<$d5>) + $($bounds)+,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
            $d4: $($bounds)+,
            $d5: $($bounds)+,
        {
            let action = Ref::new(setup);
            let name = self.name.clone();

            self.services.add(transient_factory(move |sp| {
                let config: Ref<dyn PostConfigure<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    $f4: sp.get_required::<$d4>(),
                    $f5: sp.get_required::<$d5>(),
                    _type: PhantomData,
                });
                config
            }));

            self
        }
    };
}

macro_rules! configure_methods {
    (($($bounds:tt)+)) => {
        impl<'a, T: $($bounds)+> Builder<'a, T> {
            /// Registers an action used to configure a particular type of options.
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
                    _type: PhantomData::<T>,
                };
                let action: Ref<dyn Configure<T>> = Ref::new(configure);
                let descriptor = singleton_factory(move |_| action.clone());
                self.services.add(descriptor);
                self
            }

            /// Registers an action used to post-configure a particular type of options.
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
                    _type: PhantomData,
                };
                let action: Ref<dyn PostConfigure<T>> = Ref::new(configure);
                let descriptor = singleton_factory(move |_| action.clone());
                self.services.add(descriptor);
                self
            }

            configure_builder_method!(($($bounds)+), configure1, post_configure1, _Configure1, (D, dependency));
            configure_builder_method!(($($bounds)+), configure2, post_configure2, _Configure2, (D1, dependency1), (D2, dependency2));
            configure_builder_method!(($($bounds)+), configure3, post_configure3, _Configure3, (D1, dependency1), (D2, dependency2), (D3, dependency3));
            configure_builder_method!(($($bounds)+), configure4, post_configure4, _Configure4, (D1, dependency1), (D2, dependency2), (D3, dependency3), (D4, dependency4));
            configure_builder_method!(($($bounds)+), configure5, post_configure5, _Configure5, (D1, dependency1), (D2, dependency2), (D3, dependency3), (D4, dependency4), (D5, dependency5));
        }
    };
}

cfg_if! {
    if #[cfg(feature = "async")] {
        configure_impl!((Send + Sync + 'static), _Configure, []);
        configure_impl!((Send + Sync + 'static), _Configure1, [(TDep1, dependency)]);
        configure_impl!((Send + Sync + 'static), _Configure2, [(TDep1, dependency1), (TDep2, dependency2)]);
        configure_impl!((Send + Sync + 'static), _Configure3, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3)]);
        configure_impl!((Send + Sync + 'static), _Configure4, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3), (TDep4, dependency4)]);
        configure_impl!((Send + Sync + 'static), _Configure5, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3), (TDep4, dependency4), (TDep5, dependency5)]);
        configure_methods!((Send + Sync + 'static));
    } else {
        configure_impl!(('static), _Configure, []);
        configure_impl!(('static), _Configure1, [(TDep1, dependency)]);
        configure_impl!(('static), _Configure2, [(TDep1, dependency1), (TDep2, dependency2)]);
        configure_impl!(('static), _Configure3, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3)]);
        configure_impl!(('static), _Configure4, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3), (TDep4, dependency4)]);
        configure_impl!(('static), _Configure5, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3), (TDep4, dependency4), (TDep5, dependency5)]);
        configure_methods!(('static));
    }
}

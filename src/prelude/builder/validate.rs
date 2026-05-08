use super::{names_equal, Builder};
use crate::{
    validation::{self, Validate},
    Ref, Value,
};
use cfg_if::cfg_if;
use di::{transient_factory, Ref as Svc};
use std::marker::PhantomData;

fn message_or_default<T: AsRef<str>>(message: T) -> String {
    let msg = message.as_ref();

    if msg.is_empty() {
        String::from("A validation error has occurred.")
    } else {
        String::from(msg)
    }
}

macro_rules! validate_impl {
    // base case: zero dependencies
    (($($bounds:tt)+), _Validate, validate, []) => {
        struct _Validate<TOptions, TAction> {
            name: String,
            action: Ref<TAction>,
            failure_message: String,
            _type: PhantomData<TOptions>,
        }

        impl<TOptions, TAction> Validate<TOptions> for _Validate<TOptions, TAction>
        where
            TOptions: Value,
            TAction: Fn(&TOptions) -> bool + $($bounds)+,
        {
            fn run(&self, name: &str, options: &TOptions) -> validation::Result {
                if names_equal(&self.name, name) {
                    if (self.action)(options) {
                        return validation::success();
                    } else {
                        return validation::fail(&self.failure_message);
                    }
                }

                validation::skip()
            }
        }
    };
    // n-dependency case
    (($($bounds:tt)+), $struct_name:ident, $method:ident, [$(($dep_generic:ident, $dep_field:ident)),+]) => {
        struct $struct_name<TOptions, TAction, $($dep_generic),+> {
            name: String,
            action: Ref<TAction>,
            failure_message: String,
            $($dep_field: Svc<$dep_generic>,)+
            _type: PhantomData<TOptions>,
        }

        impl<TOptions, TAction, $($dep_generic),+> Validate<TOptions>
            for $struct_name<TOptions, TAction, $($dep_generic),+>
        where
            TOptions: Value,
            TAction: Fn(&TOptions, $(Svc<$dep_generic>),+) -> bool + $($bounds)+,
            $($dep_generic: Value,)+
        {
            fn run(&self, name: &str, options: &TOptions) -> validation::Result {
                if names_equal(&self.name, name) {
                    if (self.action)(options, $(self.$dep_field.clone()),+) {
                        return validation::success();
                    } else {
                        return validation::fail(&self.failure_message);
                    }
                }

                validation::skip()
            }
        }
    };
}

// generates a builder method for validate with n-dependencies. each dep is passed as a separate group to avoid
// repetition depth mismatch with bounds.
macro_rules! validate_builder_method {
    (($($bounds:tt)+), $method:ident, $struct_name:ident, ($d1:ident, $f1:ident)) => {
        /// Registers an action used to validate a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `action` - The validation action
        /// * `failure_message` - The message used when validation fails
        pub fn $method<F, M, $d1>(self, action: F, failure_message: M) -> Self
        where
            F: Fn(&T, Svc<$d1>) -> bool + $($bounds)+,
            M: AsRef<str>,
            $d1: $($bounds)+,
        {
            let action = Ref::new(action);
            let name = self.name.clone();
            let failure_message = message_or_default(failure_message);

            self.services.add(transient_factory(move |sp| {
                let validate: Ref<dyn Validate<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    failure_message: failure_message.clone(),
                    $f1: sp.get_required::<$d1>(),
                    _type: PhantomData,
                });
                validate
            }));

            self
        }
    };
    (($($bounds:tt)+), $method:ident, $struct_name:ident, ($d1:ident, $f1:ident), ($d2:ident, $f2:ident)) => {
        /// Registers an action used to validate a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `action` - The validation action
        /// * `failure_message` - The message used when validation fails
        pub fn $method<F, M, $d1, $d2>(self, action: F, failure_message: M) -> Self
        where
            F: Fn(&T, Svc<$d1>, Svc<$d2>) -> bool + $($bounds)+,
            M: AsRef<str>,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
        {
            let action = Ref::new(action);
            let name = self.name.clone();
            let failure_message = message_or_default(failure_message);

            self.services.add(transient_factory(move |sp| {
                let validate: Ref<dyn Validate<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    failure_message: failure_message.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    _type: PhantomData,
                });
                validate
            }));

            self
        }
    };
    (($($bounds:tt)+), $method:ident, $struct_name:ident, ($d1:ident, $f1:ident), ($d2:ident, $f2:ident), ($d3:ident, $f3:ident)) => {
        /// Registers an action used to validate a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `action` - The validation action
        /// * `failure_message` - The message used when validation fails
        pub fn $method<F, M, $d1, $d2, $d3>(self, action: F, failure_message: M) -> Self
        where
            F: Fn(&T, Svc<$d1>, Svc<$d2>, Svc<$d3>) -> bool + $($bounds)+,
            M: AsRef<str>,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
        {
            let action = Ref::new(action);
            let name = self.name.clone();
            let failure_message = message_or_default(failure_message);

            self.services.add(transient_factory(move |sp| {
                let validate: Ref<dyn Validate<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    failure_message: failure_message.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    _type: PhantomData,
                });
                validate
            }));

            self
        }
    };
    (($($bounds:tt)+), $method:ident, $struct_name:ident, ($d1:ident, $f1:ident), ($d2:ident, $f2:ident), ($d3:ident, $f3:ident), ($d4:ident, $f4:ident)) => {
        /// Registers an action used to validate a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `action` - The validation action
        /// * `failure_message` - The message used when validation fails
        pub fn $method<F, M, $d1, $d2, $d3, $d4>(self, action: F, failure_message: M) -> Self
        where
            F: Fn(&T, Svc<$d1>, Svc<$d2>, Svc<$d3>, Svc<$d4>) -> bool + $($bounds)+,
            M: AsRef<str>,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
            $d4: $($bounds)+,
        {
            let action = Ref::new(action);
            let name = self.name.clone();
            let failure_message = message_or_default(failure_message);

            self.services.add(transient_factory(move |sp| {
                let validate: Ref<dyn Validate<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    failure_message: failure_message.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    $f4: sp.get_required::<$d4>(),
                    _type: PhantomData,
                });
                validate
            }));

            self
        }
    };
    (($($bounds:tt)+), $method:ident, $struct_name:ident, ($d1:ident, $f1:ident), ($d2:ident, $f2:ident), ($d3:ident, $f3:ident), ($d4:ident, $f4:ident), ($d5:ident, $f5:ident)) => {
        /// Registers an action used to validate a particular type of options.
        ///
        /// # Arguments
        ///
        /// * `action` - The validation action
        /// * `failure_message` - The message used when validation fails
        pub fn $method<F, M, $d1, $d2, $d3, $d4, $d5>(self, action: F, failure_message: M) -> Self
        where
            F: Fn(&T, Svc<$d1>, Svc<$d2>, Svc<$d3>, Svc<$d4>, Svc<$d5>) -> bool + $($bounds)+,
            M: AsRef<str>,
            $d1: $($bounds)+,
            $d2: $($bounds)+,
            $d3: $($bounds)+,
            $d4: $($bounds)+,
            $d5: $($bounds)+,
        {
            let action = Ref::new(action);
            let name = self.name.clone();
            let failure_message = message_or_default(failure_message);

            self.services.add(transient_factory(move |sp| {
                let validate: Ref<dyn Validate<T>> = Ref::new($struct_name {
                    name: name.clone(),
                    action: action.clone(),
                    failure_message: failure_message.clone(),
                    $f1: sp.get_required::<$d1>(),
                    $f2: sp.get_required::<$d2>(),
                    $f3: sp.get_required::<$d3>(),
                    $f4: sp.get_required::<$d4>(),
                    $f5: sp.get_required::<$d5>(),
                    _type: PhantomData,
                });
                validate
            }));

            self
        }
    };
}

macro_rules! validate_methods {
    (($($bounds:tt)+)) => {
        impl<'a, T: $($bounds)+> Builder<'a, T> {
            /// Registers an action used to validate a particular type of options.
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
                    let validate: Ref<dyn Validate<T>> = Ref::new(_Validate {
                        name: name.clone(),
                        action: action.clone(),
                        failure_message: failure_message.clone(),
                        _type: PhantomData,
                    });
                    validate
                }));

                self
            }

            validate_builder_method!(($($bounds)+), validate1, _Validate1, (D, dependency));
            validate_builder_method!(($($bounds)+), validate2, _Validate2, (D1, dependency1), (D2, dependency2));
            validate_builder_method!(($($bounds)+), validate3, _Validate3, (D1, dependency1), (D2, dependency2), (D3, dependency3));
            validate_builder_method!(($($bounds)+), validate4, _Validate4, (D1, dependency1), (D2, dependency2), (D3, dependency3), (D4, dependency4));
            validate_builder_method!(($($bounds)+), validate5, _Validate5, (D1, dependency1), (D2, dependency2), (D3, dependency3), (D4, dependency4), (D5, dependency5));
        }
    };
}

cfg_if! {
    if #[cfg(feature = "async")] {
        validate_impl!((Send + Sync + 'static), _Validate, validate, []);
        validate_impl!((Send + Sync + 'static), _Validate1, validate1, [(TDep1, dependency)]);
        validate_impl!((Send + Sync + 'static), _Validate2, validate2, [(TDep1, dependency1), (TDep2, dependency2)]);
        validate_impl!((Send + Sync + 'static), _Validate3, validate3, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3)]);
        validate_impl!((Send + Sync + 'static), _Validate4, validate4, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3), (TDep4, dependency4)]);
        validate_impl!((Send + Sync + 'static), _Validate5, validate5, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3), (TDep4, dependency4), (TDep5, dependency5)]);
        validate_methods!((Send + Sync + 'static));
    } else {
        validate_impl!(('static), _Validate, validate, []);
        validate_impl!(('static), _Validate1, validate1, [(TDep1, dependency)]);
        validate_impl!(('static), _Validate2, validate2, [(TDep1, dependency1), (TDep2, dependency2)]);
        validate_impl!(('static), _Validate3, validate3, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3)]);
        validate_impl!(('static), _Validate4, validate4, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3), (TDep4, dependency4)]);
        validate_impl!(('static), _Validate5, validate5, [(TDep1, dependency1), (TDep2, dependency2), (TDep3, dependency3), (TDep4, dependency4), (TDep5, dependency5)]);
        validate_methods!(('static));
    }
}

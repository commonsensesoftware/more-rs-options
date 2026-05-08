use super::Builder;
use crate::{validation::Validate, *};
use cfg_if::cfg_if;
use di::{
    exactly_one, scoped, singleton, singleton_as_self, transient, transient_factory, zero_or_more, ServiceCollection,
    ServiceDescriptor, ServiceProvider,
};

macro_rules! opts_ext {
    (($($bounds:tt)+)) => {
        /// Defines extension methods for the [ServiceCollection](::di::ServiceCollection) struct.
        pub trait OptionsExt {
            /// Registers an options type that will have all of its associated services registered.
            fn add_options<T: Value + Default + 'static>(&mut self) -> Builder<'_, T>;

            /// Registers an options type that will have all of its associated services registered.
            ///
            /// # Arguments
            ///
            /// * `name` - The name associated with the options
            fn add_named_options<T: Value + Default + 'static>(
                &mut self,
                name: impl AsRef<str>,
            ) -> Builder<'_, T>;

            /// Registers an options type that will have all of its associated services registered.
            ///
            /// # Arguments
            ///
            /// * `factory` - The function used to create the associated options factory
            fn add_options_with<T, F>(&mut self, factory: F) -> Builder<'_, T>
            where
                T: Value,
                F: Fn(&ServiceProvider) -> Ref<dyn Factory<T>> + $($bounds)+;

            /// Registers an options type that will have all of its associated services registered.
            ///
            /// # Arguments
            ///
            /// * `name` - The name associated with the options
            /// * `factory` - The function used to create the associated options factory
            fn add_named_options_with<T, F>(
                &mut self,
                name: impl AsRef<str>,
                factory: F,
            ) -> Builder<'_, T>
            where
                T: Value,
                F: Fn(&ServiceProvider) -> Ref<dyn Factory<T>> + $($bounds)+;

            /// Registers an action used to initialize a particular type of configuration options.
            ///
            /// # Arguments
            ///
            /// * `setup` - The setup action used to configure options.
            fn configure_options<T, F>(&mut self, setup: F) -> &mut Self
            where
                T: Value + Default + 'static,
                F: Fn(&mut T) + $($bounds)+;

            /// Registers an action used to initialize a particular type of configuration options.
            ///
            /// # Arguments
            ///
            /// * `name` - The name associated with the options
            /// * `setup` - The setup action used to configure options
            fn configure_named_options<T, F>(
                &mut self,
                name: impl AsRef<str>,
                setup: F,
            ) -> &mut Self
            where
                T: Value + Default + 'static,
                F: Fn(&mut T) + $($bounds)+;

            /// Registers an action used to initialize a particular type of configuration options.
            ///
            /// # Arguments
            ///
            /// * `setup` - The setup action used to configure options
            fn post_configure_options<T, F>(&mut self, setup: F) -> &mut Self
            where
                T: Value + Default + 'static,
                F: Fn(&mut T) + $($bounds)+;

            /// Registers an action used to initialize a particular type of configuration options.
            ///
            /// # Arguments
            ///
            /// * `name` - The name associated with the options
            /// * `setup` - The setup action used to configure options
            fn post_configure_named_options<T, F>(
                &mut self,
                name: impl AsRef<str>,
                setup: F,
            ) -> &mut Self
            where
                T: Value + Default + 'static,
                F: Fn(&mut T) + $($bounds)+;
        }
    };
}

fn _add_options<'a, T: Value>(
    services: &'a mut ServiceCollection,
    name: &str,
    descriptor: ServiceDescriptor,
) -> Builder<'a, T> {
    services
        .try_add(
            singleton_as_self::<Manager<T>>()
                .depends_on(exactly_one::<dyn Factory<T>>())
                .from(|sp| Ref::new(Manager::new(sp.get_required::<dyn Factory<T>>()))),
        )
        .try_add(
            singleton::<T, Manager<T>>()
                .depends_on(exactly_one::<Manager<T>>())
                .from(|sp| sp.get_required::<Manager<T>>().get_unchecked()),
        )
        .try_add(
            scoped::<dyn Snapshot<T>, Manager<T>>()
                .depends_on(exactly_one::<Manager<T>>())
                .from(|sp| sp.get_required::<Manager<T>>()),
        )
        .try_add(
            singleton::<dyn Monitor<T>, DefaultMonitor<T>>()
                .depends_on(exactly_one::<dyn MonitorCache<T>>())
                .depends_on(zero_or_more::<dyn ChangeTokenSource<T>>())
                .depends_on(exactly_one::<dyn Factory<T>>())
                .from(|sp| {
                    Ref::new(DefaultMonitor::new(
                        sp.get_required::<dyn MonitorCache<T>>(),
                        sp.get_all::<dyn ChangeTokenSource<T>>().collect(),
                        sp.get_required::<dyn Factory<T>>(),
                    ))
                }),
        )
        .try_add(descriptor)
        .try_add(singleton::<dyn MonitorCache<T>, Cache<T>>().from(|_| Ref::new(Cache::default())));

    Builder::new(services, name)
}

macro_rules! opts_ext_impl {
    (($($bounds:tt)+)) => {
        impl OptionsExt for ServiceCollection {
            fn add_options<T: Value + Default + 'static>(&mut self) -> Builder<'_, T> {
                let descriptor = transient::<dyn Factory<T>, DefaultFactory<T>>()
                    .depends_on(zero_or_more::<dyn Configure<T>>())
                    .depends_on(zero_or_more::<dyn PostConfigure<T>>())
                    .depends_on(zero_or_more::<dyn Validate<T>>())
                    .from(|sp| {
                        Ref::new(DefaultFactory::new(
                            sp.get_all::<dyn Configure<T>>().collect(),
                            sp.get_all::<dyn PostConfigure<T>>().collect(),
                            sp.get_all::<dyn Validate<T>>().collect(),
                        ))
                    });

                _add_options(self, "", descriptor)
            }

            fn add_named_options<T: Value + Default + 'static>(
                &mut self,
                name: impl AsRef<str>,
            ) -> Builder<'_, T> {
                let descriptor = transient::<dyn Factory<T>, DefaultFactory<T>>()
                    .depends_on(zero_or_more::<dyn Configure<T>>())
                    .depends_on(zero_or_more::<dyn PostConfigure<T>>())
                    .depends_on(zero_or_more::<dyn Validate<T>>())
                    .from(|sp| {
                        Ref::new(DefaultFactory::new(
                            sp.get_all::<dyn Configure<T>>().collect(),
                            sp.get_all::<dyn PostConfigure<T>>().collect(),
                            sp.get_all::<dyn Validate<T>>().collect(),
                        ))
                    });

                _add_options(self, name.as_ref(), descriptor)
            }

            #[inline]
            fn add_options_with<T, F>(&mut self, factory: F) -> Builder<'_, T>
            where
                T: Value,
                F: Fn(&ServiceProvider) -> Ref<dyn Factory<T>> + $($bounds)+,
            {
                _add_options(self, "", transient_factory(factory))
            }

            #[inline]
            fn add_named_options_with<T, F>(
                &mut self,
                name: impl AsRef<str>,
                factory: F,
            ) -> Builder<'_, T>
            where
                T: Value,
                F: Fn(&ServiceProvider) -> Ref<dyn Factory<T>> + $($bounds)+,
            {
                _add_options(self, name.as_ref(), transient_factory(factory))
            }

            #[inline]
            fn configure_options<T, F>(&mut self, setup: F) -> &mut Self
            where
                T: Value + Default + 'static,
                F: Fn(&mut T) + $($bounds)+,
            {
                self.add_options().configure(setup).into()
            }

            #[inline]
            fn configure_named_options<T, F>(
                &mut self,
                name: impl AsRef<str>,
                setup: F,
            ) -> &mut Self
            where
                T: Value + Default + 'static,
                F: Fn(&mut T) + $($bounds)+,
            {
                self.add_named_options(name).configure(setup).into()
            }

            #[inline]
            fn post_configure_options<T, F>(&mut self, setup: F) -> &mut Self
            where
                T: Value + Default + 'static,
                F: Fn(&mut T) + $($bounds)+,
            {
                self.add_options().post_configure(setup).into()
            }

            #[inline]
            fn post_configure_named_options<T, F>(
                &mut self,
                name: impl AsRef<str>,
                setup: F,
            ) -> &mut Self
            where
                T: Value + Default + 'static,
                F: Fn(&mut T) + $($bounds)+,
            {
                self.add_named_options(name).post_configure(setup).into()
            }
        }
    };
}

cfg_if! {
    if #[cfg(feature = "async")] {
        opts_ext!((Send + Sync + 'static));
        opts_ext_impl!((Send + Sync + 'static));
    } else {
        opts_ext!(('static));
        opts_ext_impl!(('static));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use di::{existing_as_self, transient};
    use std::sync::RwLock;

    #[derive(Default, Debug, PartialEq, Eq)]
    struct TestOptions {
        enabled: bool,
        setting: usize,
    }

    #[derive(Default)]
    struct TestValidation;

    impl Validate<TestOptions> for TestValidation {
        fn run(&self, _name: &str, options: &TestOptions) -> validation::Result {
            if !options.enabled && options.setting > 0 {
                validation::fail("Setting must be zero when disabled")
            } else {
                validation::success()
            }
        }
    }

    struct TestService {
        value: RwLock<usize>,
    }

    impl TestService {
        fn next(&self) -> usize {
            let mut value = self.value.write().unwrap();
            let current = *value;
            *value += 1;
            current
        }

        fn calls(&self) -> usize {
            *self.value.read().unwrap() - 1
        }
    }

    impl Default for TestService {
        fn default() -> Self {
            Self { value: RwLock::new(1) }
        }
    }

    #[test]
    fn get_should_resolve_service() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .build_provider()
            .unwrap();

        // act
        let result = provider.get::<TestOptions>();

        // assert
        assert!(result.is_some());
    }

    #[test]
    fn get_required_should_configure_options() {
        // arrange
        let provider = ServiceCollection::new()
            .configure_options(|o: &mut TestOptions| o.setting = 1)
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 1);
    }

    #[test]
    fn get_required_should_post_configure_options() {
        // arrange
        let provider = ServiceCollection::new()
            .post_configure_options(|o: &mut TestOptions| o.setting = 1)
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 1);
    }

    #[test]
    fn get_required_should_apply_all_configurations() {
        // arrange
        let provider = ServiceCollection::new()
            .configure_options(|o: &mut TestOptions| o.setting = 1)
            .configure_options(|o: &mut TestOptions| o.enabled = true)
            .post_configure_options(|o: &mut TestOptions| o.setting = 2)
            .build_provider()
            .unwrap();

        // act
        let result = provider.get_required::<TestOptions>();
        let options = result;

        // assert
        assert!(options.enabled);
        assert_eq!(options.setting, 2);
    }

    #[test]
    fn get_required_should_not_panic_when_configured_options_are_valid() {
        // arrange
        let provider = ServiceCollection::new()
            .configure_options(|o: &mut TestOptions| {
                o.enabled = true;
                o.setting = 1;
            })
            .add(transient::<dyn Validate<TestOptions>, TestValidation>().from(|_| Ref::new(TestValidation::default())))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        let _ = options;
    }

    #[test]
    #[should_panic(expected = "Setting must be zero when disabled")]
    fn get_required_should_panic_when_configured_options_are_invalid() {
        // arrange
        let provider = ServiceCollection::new()
            .configure_options(|o: &mut TestOptions| {
                o.enabled = false;
                o.setting = 1;
            })
            .add(transient::<dyn Validate<TestOptions>, TestValidation>().from(|_| Ref::new(TestValidation::default())))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        let _ = options;
    }

    #[test]
    fn get_required_should_configure_options_with_1_dependency() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure1(|o, d1: Ref<TestService>| o.setting = d1.next())
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 1);
    }

    #[test]
    fn get_required_should_configure_options_with_2_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure2(|o, d1: Ref<TestService>, d2: Ref<TestService>| o.setting = d1.next() + d2.next())
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 3);
    }

    #[test]
    fn get_required_should_configure_options_with_3_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure3(|o, d1: Ref<TestService>, d2: Ref<TestService>, d3: Ref<TestService>| {
                o.setting = d1.next() + d2.next() + d3.next()
            })
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 6);
    }

    #[test]
    fn get_required_should_configure_options_with_4_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure4(
                |o, d1: Ref<TestService>, d2: Ref<TestService>, d3: Ref<TestService>, d4: Ref<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next() + d4.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 10);
    }

    #[test]
    fn get_required_should_configure_options_with_5_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure5(
                |o,
                 d1: Ref<TestService>,
                 d2: Ref<TestService>,
                 d3: Ref<TestService>,
                 d4: Ref<TestService>,
                 d5: Ref<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next() + d4.next() + d5.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 15);
    }

    #[test]
    fn get_required_should_post_configure_options_with_1_dependency() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure1(|o, d1: Ref<TestService>| o.setting = d1.next())
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 1);
    }

    #[test]
    fn get_required_should_post_configure_options_with_2_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure2(|o, d1: Ref<TestService>, d2: Ref<TestService>| o.setting = d1.next() + d2.next())
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 3);
    }

    #[test]
    fn get_required_should_post_configure_options_with_3_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure3(|o, d1: Ref<TestService>, d2: Ref<TestService>, d3: Ref<TestService>| {
                o.setting = d1.next() + d2.next() + d3.next()
            })
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 6);
    }

    #[test]
    fn get_required_should_post_configure_options_with_4_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure4(
                |o, d1: Ref<TestService>, d2: Ref<TestService>, d3: Ref<TestService>, d4: Ref<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next() + d4.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 10);
    }

    #[test]
    fn get_required_should_post_configure_options_with_5_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure5(
                |o,
                 d1: Ref<TestService>,
                 d2: Ref<TestService>,
                 d3: Ref<TestService>,
                 d4: Ref<TestService>,
                 d5: Ref<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next() + d4.next() + d5.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();

        // assert
        assert_eq!(options.setting, 15);
    }

    #[test]
    fn get_required_should_validate_options_with_1_dependency() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate1(
                |o, d1: Ref<TestService>| {
                    let _ = d1.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.enabled, true);
        assert_eq!(service.calls(), 1);
    }

    #[test]
    fn get_required_should_validate_options_with_2_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate2(
                |o, d1: Ref<TestService>, d2: Ref<TestService>| {
                    let _ = d1.next() + d2.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.enabled, true);
        assert_eq!(service.calls(), 2);
    }

    #[test]
    fn get_required_should_validate_options_with_3_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate3(
                |o, d1: Ref<TestService>, d2: Ref<TestService>, d3: Ref<TestService>| {
                    let _ = d1.next() + d2.next() + d3.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.enabled, true);
        assert_eq!(service.calls(), 3);
    }

    #[test]
    fn get_required_should_validate_options_with_4_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate4(
                |o, d1: Ref<TestService>, d2: Ref<TestService>, d3: Ref<TestService>, d4: Ref<TestService>| {
                    let _ = d1.next() + d2.next() + d3.next() + d4.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.enabled, true);
        assert_eq!(service.calls(), 4);
    }

    #[test]
    fn get_required_should_validate_options_with_5_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate5(
                |o,
                 d1: Ref<TestService>,
                 d2: Ref<TestService>,
                 d3: Ref<TestService>,
                 d4: Ref<TestService>,
                 d5: Ref<TestService>| {
                    let _ = d1.next() + d2.next() + d3.next() + d4.next() + d5.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<TestOptions>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.enabled, true);
        assert_eq!(service.calls(), 5);
    }
}

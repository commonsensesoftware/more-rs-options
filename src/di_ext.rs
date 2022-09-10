use crate::*;
use di::*;

/// Defines extension methods for the `ServiceCollection` struct.
pub trait OptionsServiceExtensions {
    /// Registers an options type that will have all of its associated services registered.
    fn add_options<T: Default + 'static>(&mut self) -> OptionsBuilder<T>;

    /// Registers an options type that will have all of its associated services registered.
    ///
    /// # Arguments
    ///
    /// * `name` - the name associated with the options
    fn add_named_options<T: Default + 'static>(
        &mut self,
        name: impl AsRef<str>,
    ) -> OptionsBuilder<T>;

    /// Registers an options type that will have all of its associated services registered.
    ///
    /// # Arguments
    ///
    /// * `factory` - the function used to create the associated options factory
    fn add_options_with<T, F>(&mut self, factory: F) -> OptionsBuilder<T>
    where
        F: Fn(&ServiceProvider) -> ServiceRef<dyn OptionsFactory<T>> + 'static;

    /// Registers an options type that will have all of its associated services registered.
    ///
    /// # Arguments
    ///
    /// * `name` - the name associated with the options
    /// * `factory` - the function used to create the associated options factory
    fn add_named_options_with<T, F>(
        &mut self,
        name: impl AsRef<str>,
        factory: F,
    ) -> OptionsBuilder<T>
    where
        F: Fn(&ServiceProvider) -> ServiceRef<dyn OptionsFactory<T>> + 'static;

    /// Registers an action used to initialize a particular type of configuration options.
    ///
    /// # Arguments
    ///
    /// * `setup` - The setup action used to configure options.
    fn configure_options<T: Default + 'static, F>(&mut self, setup: F) -> &mut Self
    where
        F: Fn(&mut T) + 'static;

    /// Registers an action used to initialize a particular type of configuration options.
    ///
    /// # Arguments
    ///
    /// * `name` - the name associated with the options
    /// * `setup` - The setup action used to configure options
    fn configure_named_options<T: Default + 'static, F>(
        &mut self,
        name: impl AsRef<str>,
        setup: F,
    ) -> &mut Self
    where
        F: Fn(&mut T) + 'static;

    /// Registers an action used to initialize a particular type of configuration options.
    ///
    /// # Arguments
    ///
    /// * `setup` - The setup action used to configure options
    fn post_configure_options<T: Default + 'static, F>(&mut self, setup: F) -> &mut Self
    where
        F: Fn(&mut T) + 'static;

    /// Registers an action used to initialize a particular type of configuration options.
    ///
    /// # Arguments
    ///
    /// * `name` - the name associated with the options
    /// * `setup` - The setup action used to configure options
    fn post_configure_named_options<T: Default + 'static, F>(
        &mut self,
        name: impl AsRef<str>,
        setup: F,
    ) -> &mut Self
    where
        F: Fn(&mut T) + 'static;
}

fn _add_options<'a, T>(
    services: &'a mut ServiceCollection,
    name: Option<&str>,
    descriptor: ServiceDescriptor,
) -> OptionsBuilder<'a, T> {
    services
        .try_add(singleton_as_self::<OptionsManager<T>>().from(|sp| {
            ServiceRef::new(OptionsManager::new(
                sp.get_required::<dyn OptionsFactory<T>>(),
            ))
        }))
        .try_add(
            singleton::<dyn Options<T>, OptionsManager<T>>()
                .from(|sp| sp.get_required::<OptionsManager<T>>()),
        )
        .try_add(
            scoped::<dyn OptionsSnapshot<T>, OptionsManager<T>>()
                .from(|sp| sp.get_required::<OptionsManager<T>>()),
        )
        .try_add(singleton_factory(|sp| {
            let monitor: ServiceRef<dyn OptionsMonitor<T>> =
                ServiceRef::new(DefaultOptionsMonitor::new(
                    sp.get_required::<dyn OptionsMonitorCache<T>>(),
                    sp.get_all::<dyn OptionsChangeTokenSource<T>>().collect(),
                    sp.get_required::<dyn OptionsFactory<T>>(),
                ));
            monitor
        }))
        .try_add(descriptor)
        .try_add(
            singleton::<dyn OptionsMonitorCache<T>, OptionsCache<T>>()
                .from(|_| ServiceRef::new(OptionsCache::default())),
        );

    OptionsBuilder::new(services, name)
}

impl OptionsServiceExtensions for ServiceCollection {
    fn add_options<T: Default + 'static>(&mut self) -> OptionsBuilder<T> {
        let descriptor = transient_factory(|sp| {
            let factory: ServiceRef<dyn OptionsFactory<T>> =
                ServiceRef::new(DefaultOptionsFactory::new(
                    sp.get_all::<dyn ConfigureOptions<T>>().collect(),
                    sp.get_all::<dyn PostConfigureOptions<T>>().collect(),
                    sp.get_all::<dyn ValidateOptions<T>>().collect(),
                ));
            factory
        });

        _add_options(self, None, descriptor)
    }

    fn add_named_options<T: Default + 'static>(
        &mut self,
        name: impl AsRef<str>,
    ) -> OptionsBuilder<T> {
        let descriptor = transient_factory(|sp| {
            let factory: ServiceRef<dyn OptionsFactory<T>> =
                ServiceRef::new(DefaultOptionsFactory::new(
                    sp.get_all::<dyn ConfigureOptions<T>>().collect(),
                    sp.get_all::<dyn PostConfigureOptions<T>>().collect(),
                    sp.get_all::<dyn ValidateOptions<T>>().collect(),
                ));
            factory
        });

        _add_options(self, Some(name.as_ref()), descriptor)
    }

    fn add_options_with<T, F>(&mut self, factory: F) -> OptionsBuilder<T>
    where
        F: Fn(&ServiceProvider) -> ServiceRef<dyn OptionsFactory<T>> + 'static,
    {
        _add_options(self, None, transient_factory(factory))
    }

    fn add_named_options_with<T, F>(
        &mut self,
        name: impl AsRef<str>,
        factory: F,
    ) -> OptionsBuilder<T>
    where
        F: Fn(&ServiceProvider) -> ServiceRef<dyn OptionsFactory<T>> + 'static,
    {
        _add_options(self, Some(name.as_ref()), transient_factory(factory))
    }

    fn configure_options<T: Default + 'static, F>(&mut self, setup: F) -> &mut Self
    where
        F: Fn(&mut T) + 'static,
    {
        self.add_options().configure(setup).into()
    }

    fn configure_named_options<T: Default + 'static, F>(
        &mut self,
        name: impl AsRef<str>,
        setup: F,
    ) -> &mut Self
    where
        F: Fn(&mut T) + 'static,
    {
        self.add_named_options(name).configure(setup).into()
    }

    fn post_configure_options<T: Default + 'static, F>(&mut self, setup: F) -> &mut Self
    where
        F: Fn(&mut T) + 'static,
    {
        self.add_options().post_configure(setup).into()
    }

    fn post_configure_named_options<T: Default + 'static, F>(
        &mut self,
        name: impl AsRef<str>,
        setup: F,
    ) -> &mut Self
    where
        F: Fn(&mut T) + 'static,
    {
        self.add_named_options(name).configure(setup).into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use di::{existing_as_self, transient};
    use std::cell::Cell;

    #[derive(Default, Debug, PartialEq, Eq)]
    struct TestOptions {
        enabled: bool,
        setting: usize,
    }

    #[derive(Default)]
    struct TestValidation;

    impl ValidateOptions<TestOptions> for TestValidation {
        fn validate(&self, _name: Option<&str>, options: &TestOptions) -> ValidateOptionsResult {
            if !options.enabled && options.setting > 0 {
                ValidateOptionsResult::fail("Setting must be zero when disabled")
            } else {
                ValidateOptionsResult::success()
            }
        }
    }

    struct TestService {
        value: Cell<usize>,
    }

    impl TestService {
        fn next(&self) -> usize {
            self.value.replace(self.value.get() + 1)
        }

        fn calls(&self) -> usize {
            self.value.get() - 1
        }
    }

    impl Default for TestService {
        fn default() -> Self {
            Self {
                value: Cell::new(1),
            }
        }
    }

    #[test]
    fn get_should_resolve_service() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .build_provider();

        // act
        let result = provider.get::<dyn Options<TestOptions>>();

        // assert
        assert!(result.is_some());
    }

    #[test]
    fn get_required_should_configure_options() {
        // arrange
        let provider = ServiceCollection::new()
            .configure_options(|o: &mut TestOptions| o.setting = 1)
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 1);
    }

    #[test]
    fn get_required_should_post_configure_options() {
        // arrange
        let provider = ServiceCollection::new()
            .post_configure_options(|o: &mut TestOptions| o.setting = 1)
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 1);
    }

    #[test]
    fn get_required_should_apply_all_configurations() {
        // arrange
        let provider = ServiceCollection::new()
            .configure_options(|o: &mut TestOptions| o.setting = 1)
            .configure_options(|o: &mut TestOptions| o.enabled = true)
            .post_configure_options(|o: &mut TestOptions| o.setting = 2)
            .build_provider();

        // act
        let result = provider.get_required::<dyn Options<TestOptions>>();
        let options = result.value();

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
            .add(
                transient::<dyn ValidateOptions<TestOptions>, TestValidation>()
                    .from(|_| ServiceRef::new(TestValidation::default())),
            )
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        let _ = options.value();
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
            .add(
                transient::<dyn ValidateOptions<TestOptions>, TestValidation>()
                    .from(|_| ServiceRef::new(TestValidation::default())),
            )
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        let _ = options.value();
    }

    #[test]
    fn get_required_should_configure_options_with_1_dependency() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure1(|o, d1: ServiceRef<TestService>| o.setting = d1.next())
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 1);
    }

    #[test]
    fn get_required_should_configure_options_with_2_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure2(
                |o, d1: ServiceRef<TestService>, d2: ServiceRef<TestService>| {
                    o.setting = d1.next() + d2.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 3);
    }

    #[test]
    fn get_required_should_configure_options_with_3_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure3(
                |o,
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 6);
    }

    #[test]
    fn get_required_should_configure_options_with_4_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure4(
                |o,
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>,
                 d4: ServiceRef<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next() + d4.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 10);
    }

    #[test]
    fn get_required_should_configure_options_with_5_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure5(
                |o,
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>,
                 d4: ServiceRef<TestService>,
                 d5: ServiceRef<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next() + d4.next() + d5.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 15);
    }

    #[test]
    fn get_required_should_post_configure_options_with_1_dependency() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure1(|o, d1: ServiceRef<TestService>| o.setting = d1.next())
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 1);
    }

    #[test]
    fn get_required_should_post_configure_options_with_2_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure2(
                |o, d1: ServiceRef<TestService>, d2: ServiceRef<TestService>| {
                    o.setting = d1.next() + d2.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 3);
    }

    #[test]
    fn get_required_should_post_configure_options_with_3_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure3(
                |o,
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 6);
    }

    #[test]
    fn get_required_should_post_configure_options_with_4_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure4(
                |o,
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>,
                 d4: ServiceRef<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next() + d4.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 10);
    }

    #[test]
    fn get_required_should_post_configure_options_with_5_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .post_configure5(
                |o,
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>,
                 d4: ServiceRef<TestService>,
                 d5: ServiceRef<TestService>| {
                    o.setting = d1.next() + d2.next() + d3.next() + d4.next() + d5.next()
                },
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert_eq!(options.value().setting, 15);
    }

    #[test]
    fn get_required_should_validate_options_with_1_dependency() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate1(
                |o, d1: ServiceRef<TestService>| {
                    let _ = d1.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.value().enabled, true);
        assert_eq!(service.calls(), 1);
    }

    #[test]
    fn get_required_should_validate_options_with_2_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate2(
                |o, d1: ServiceRef<TestService>, d2: ServiceRef<TestService>| {
                    let _ = d1.next() + d2.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.value().enabled, true);
        assert_eq!(service.calls(), 2);
    }

    #[test]
    fn get_required_should_validate_options_with_3_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate3(
                |o,
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>| {
                    let _ = d1.next() + d2.next() + d3.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.value().enabled, true);
        assert_eq!(service.calls(), 3);
    }

    #[test]
    fn get_required_should_validate_options_with_4_dependencies() {
        // arrange
        let provider = ServiceCollection::new()
            .add_options::<TestOptions>()
            .configure(|o| o.enabled = true)
            .validate4(
                |o,
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>,
                 d4: ServiceRef<TestService>| {
                    let _ = d1.next() + d2.next() + d3.next() + d4.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.value().enabled, true);
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
                 d1: ServiceRef<TestService>,
                 d2: ServiceRef<TestService>,
                 d3: ServiceRef<TestService>,
                 d4: ServiceRef<TestService>,
                 d5: ServiceRef<TestService>| {
                    let _ = d1.next() + d2.next() + d3.next() + d4.next() + d5.next();
                    o.enabled
                },
                "Not enabled!",
            )
            .add(existing_as_self(TestService::default()))
            .build_provider();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();
        let service = provider.get_required::<TestService>();

        // assert
        assert_eq!(options.value().enabled, true);
        assert_eq!(service.calls(), 5);
    }
}

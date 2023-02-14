use crate::{ext::*, *};
use config::ext::*;
use config::Configuration;
use di::{existing, ServiceCollection, ServiceRef};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use tokens::ChangeToken;

/// Represents a change token for monitored options that are notified when configuration changes.
pub struct ConfigurationChangeTokenSource<TOptions> {
    name: Option<String>,
    configuration: ServiceRef<dyn Configuration>,
    _data: PhantomData<TOptions>,
}

impl<TOptions> ConfigurationChangeTokenSource<TOptions> {
    /// Initializes a new configuration change token source.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options being watched
    /// * `configuration` - The source configuration
    pub fn new(name: Option<&str>, configuration: ServiceRef<dyn Configuration>) -> Self {
        Self {
            name: name.map(|s| s.to_owned()),
            configuration,
            _data: PhantomData,
        }
    }
}

impl<TOptions> OptionsChangeTokenSource<TOptions> for ConfigurationChangeTokenSource<TOptions> {
    fn token(&self) -> Box<dyn ChangeToken> {
        self.configuration.reload_token()
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// Defines extension methods for the `ServiceCollection` struct.
pub trait OptionsConfigurationServiceExtensions {
    /// Registers an options type that will have all of its associated services registered.
    fn apply_config<T>(
        &mut self,
        configuration: ServiceRef<dyn Configuration>,
    ) -> OptionsBuilder<T>
    where
        T: Default + DeserializeOwned + 'static;

    /// Registers an options type that will have all of its associated services registered.
    fn apply_config_at<T>(
        &mut self,
        configuration: ServiceRef<dyn Configuration>,
        key: impl AsRef<str>,
    ) -> OptionsBuilder<T>
    where
        T: Default + DeserializeOwned + 'static;
}

impl OptionsConfigurationServiceExtensions for ServiceCollection {
    fn apply_config<T>(&mut self, configuration: ServiceRef<dyn Configuration>) -> OptionsBuilder<T>
    where
        T: Default + DeserializeOwned + 'static,
    {
        let source = Box::new(ConfigurationChangeTokenSource::<T>::new(
            None,
            configuration.clone(),
        ));
        let descriptor =
            existing::<dyn OptionsChangeTokenSource<T>, ConfigurationChangeTokenSource<T>>(source);

        self.add(descriptor)
            .add_options()
            .configure(move |options: &mut T| configuration.bind(options))
    }

    fn apply_config_at<T>(
        &mut self,
        configuration: ServiceRef<dyn Configuration>,
        key: impl AsRef<str>,
    ) -> OptionsBuilder<T>
    where
        T: Default + DeserializeOwned + 'static,
    {
        let source = Box::new(ConfigurationChangeTokenSource::<T>::new(
            Some(key.as_ref()),
            configuration.clone(),
        ));
        let descriptor =
            existing::<dyn OptionsChangeTokenSource<T>, ConfigurationChangeTokenSource<T>>(source);
        let key = key.as_ref().to_owned();

        self.add(descriptor)
            .add_named_options(&key)
            .configure(move |options: &mut T| configuration.bind_at(&key, options))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use config::{ConfigurationBuilder, DefaultConfigurationBuilder};
    use di::ServiceCollection;
    use serde::Deserialize;

    #[derive(Default, Deserialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    struct TestOptions {
        enabled: bool,
    }

    #[test]
    fn apply_config_should_bind_configuration_to_options() {
        // arrange
        let config = ServiceRef::from(
            DefaultConfigurationBuilder::new()
                .add_in_memory(
                    [("Enabled", "true")]
                        .iter()
                        .map(|t| (t.0.to_owned(), t.1.to_owned()))
                        .collect(),
                )
                .build()
                .as_config(),
        );
        let provider = ServiceCollection::new()
            .apply_config::<TestOptions>(config)
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<dyn Options<TestOptions>>();

        // assert
        assert!(options.value().enabled);
    }

    #[test]
    fn apply_config_at_should_bind_configuration_to_options() {
        // arrange
        let config = ServiceRef::from(
            DefaultConfigurationBuilder::new()
                .add_in_memory(
                    [("Test:Enabled", "true")]
                        .iter()
                        .map(|t| (t.0.to_owned(), t.1.to_owned()))
                        .collect(),
                )
                .build()
                .as_config(),
        );
        let provider = ServiceCollection::new()
            .apply_config_at::<TestOptions>(config, "Test")
            .build_provider()
            .unwrap();

        // act
        let options = provider.get_required::<dyn OptionsSnapshot<TestOptions>>();

        // assert
        assert!(options.get(Some("Test")).enabled);
    }
}

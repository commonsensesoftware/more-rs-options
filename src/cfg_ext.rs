use crate::{ext::*, *};
use config::ext::*;
use config::Configuration;
use di::{existing, Ref, ServiceCollection};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use tokens::ChangeToken;

/// Represents a change token for monitored [`Options`](crate::Options) that are
/// notified when configuration changes.
pub struct ConfigurationChangeTokenSource<T: Value> {
    name: Option<String>,
    configuration: Ref<dyn Configuration>,
    _data: PhantomData<T>,
}

unsafe impl<T: Send + Sync> Send for ConfigurationChangeTokenSource<T> {}
unsafe impl<T: Send + Sync> Sync for ConfigurationChangeTokenSource<T> {}

impl<T: Value> ConfigurationChangeTokenSource<T> {
    /// Initializes a new configuration change token source.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options being watched
    /// * `configuration` - The source [configuration](config::Configuration)
    pub fn new(name: Option<&str>, configuration: Ref<dyn Configuration>) -> Self {
        Self {
            name: name.map(|s| s.to_owned()),
            configuration,
            _data: PhantomData,
        }
    }
}

impl<T: Value> OptionsChangeTokenSource<T> for ConfigurationChangeTokenSource<T> {
    fn token(&self) -> Box<dyn ChangeToken> {
        self.configuration.reload_token()
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// Defines extension methods for the [`ServiceCollection`](di::ServiceCollection) struct.
pub trait OptionsConfigurationServiceExtensions {
    /// Registers an options type that will have all of its associated services registered.
    ///
    /// # Arguments
    ///
    /// * `configuration` - The [configuration](config::Configuration) applied to the options
    fn apply_config<T>(&mut self, configuration: Ref<dyn Configuration>) -> OptionsBuilder<T>
    where
        T: Value + Default + DeserializeOwned + 'static;

    /// Registers an options type that will have all of its associated services registered.
    ///
    /// # Arguments
    ///
    /// * `configuration` - The [configuration](config::Configuration) applied to the options
    /// * `key` - The key to the part of the [configuration](config::Configuration) applied to the options
    fn apply_config_at<T>(
        &mut self,
        configuration: Ref<dyn Configuration>,
        key: impl AsRef<str>,
    ) -> OptionsBuilder<T>
    where
        T: Value + Default + DeserializeOwned + 'static;
}

impl OptionsConfigurationServiceExtensions for ServiceCollection {
    fn apply_config<T>(&mut self, configuration: Ref<dyn Configuration>) -> OptionsBuilder<T>
    where
        T: Value + Default + DeserializeOwned + 'static,
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
        configuration: Ref<dyn Configuration>,
        key: impl AsRef<str>,
    ) -> OptionsBuilder<T>
    where
        T: Value + Default + DeserializeOwned + 'static,
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
    use serde_json::json;
    use std::env::temp_dir;
    use std::fs::{remove_file, File};
    use std::io::Write;
    use std::sync::{Arc, Condvar, Mutex};
    use std::time::Duration;

    #[derive(Default, Deserialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    struct TestOptions {
        enabled: bool,
    }

    #[test]
    fn apply_config_should_bind_configuration_to_options() {
        // arrange
        let config = Ref::from(
            DefaultConfigurationBuilder::new()
                .add_in_memory(&[("Enabled", "true")])
                .build()
                .unwrap()
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
    fn apply_config_should_bind_configuration_section_to_options() {
        // arrange
        let config = DefaultConfigurationBuilder::new()
            .add_in_memory(&[("Test:Enabled", "true")])
            .build()
            .unwrap();
        let provider = ServiceCollection::new()
            .apply_config::<TestOptions>(config.section("Test").as_config().into())
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
        let config = Ref::from(
            DefaultConfigurationBuilder::new()
                .add_in_memory(&[("Test:Enabled", "true")])
                .build()
                .unwrap()
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

    #[test]
    fn options_should_be_updated_after_configuration_change() {
        // arrange
        let path = temp_dir().join("options_from_json_1.json");
        let mut json = json!({"enabled": true});

        let mut file = File::create(&path).unwrap();
        file.write_all(json.to_string().as_bytes()).unwrap();
        drop(file);

        let config: Ref<dyn Configuration> = Ref::from(
            DefaultConfigurationBuilder::new()
                .add_json_file(&path.is().reloadable())
                .build()
                .unwrap()
                .as_config(),
        );
        let provider = ServiceCollection::new()
            .apply_config::<TestOptions>(config.clone())
            .build_provider()
            .unwrap();

        let token = config.reload_token();
        let original = provider
            .get_required::<dyn OptionsMonitor<TestOptions>>()
            .current_value();
        let state = Arc::new((Mutex::new(false), Condvar::new()));
        let _unused = token.register(
            Box::new(|s| {
                let data = s.unwrap();
                let (reloaded, event) = &*(data.downcast_ref::<(Mutex<bool>, Condvar)>().unwrap());
                *reloaded.lock().unwrap() = true;
                event.notify_one();
            }),
            Some(state.clone()),
        );

        json = json!({"enabled": false});
        file = File::create(&path).unwrap();
        file.write_all(json.to_string().as_bytes()).unwrap();
        drop(file);

        let (mutex, event) = &*state;
        let mut reloaded = mutex.lock().unwrap();

        while !*reloaded {
            reloaded = event
                .wait_timeout(reloaded, Duration::from_secs(1))
                .unwrap()
                .0;
        }

        // act
        let current = provider
            .get_required::<dyn OptionsMonitor<TestOptions>>()
            .current_value();

        // assert
        if path.exists() {
            remove_file(&path).ok();
        }

        assert_eq!(original.enabled, true);
        assert_eq!(current.enabled, false);
    }
}

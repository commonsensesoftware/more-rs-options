use crate::{prelude::*, OptionsChangeTokenSource, Value};
use ::config::{prelude::*, Reloadable};
use ::di::{transient_factory, ServiceCollection};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use tokens::{ChangeToken, NeverChangeToken};

struct NeverReloads;

impl Reloadable for NeverReloads {
    #[inline]
    fn can_reload(&self) -> bool {
        false
    }

    #[inline]
    fn reload_token(&self) -> impl ChangeToken + 'static {
        NeverChangeToken
    }
}

struct ConfigChangeTokenSource<R, V> {
    name: Option<String>,
    reloadable: R,
    _data: PhantomData<V>,
}

impl<R, T> ConfigChangeTokenSource<R, T> {
    #[inline]
    pub fn new(name: Option<String>, reloadable: R) -> Self {
        Self {
            name,
            reloadable,
            _data: PhantomData,
        }
    }
}

impl<R: Value + Reloadable, T: Value> OptionsChangeTokenSource<T> for ConfigChangeTokenSource<R, T> {
    #[inline]
    fn token(&self) -> Box<dyn ChangeToken> {
        Box::new(self.reloadable.reload_token())
    }

    #[inline]
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// Defines configuration extension methods for a [ServiceCollection](::di::ServiceCollection).
pub trait ConfigExt<C>: Sized {
    /// Registers an options type that will have all of its associated services registered.
    ///
    /// # Arguments
    ///
    /// * `configuration` - The [configuration](config::Configuration) applied to the options
    fn apply_config<T>(&mut self, configuration: C) -> OptionsBuilder<'_, T>
    where
        T: Value + Default + DeserializeOwned + Send + Sync + 'static;

    /// Registers an options type that will have all of its associated services registered.
    ///
    /// # Arguments
    ///
    /// * `configuration` - The [configuration](config::Configuration) applied to the options
    /// * `key` - The key to the part of the [configuration](config::Configuration) applied to the options
    fn apply_config_at<T>(&mut self, configuration: C, key: impl AsRef<str>) -> OptionsBuilder<'_, T>
    where
        T: Value + Default + DeserializeOwned + Send + Sync + 'static;
}

fn add_change_token_source<B, T>(services: &mut ServiceCollection, name: Option<String>, configuration: &B)
where
    B: Value + Reloadable + Clone + 'static,
    T: Value + 'static,
{
    if configuration.can_reload() {
        let reloadable = configuration.clone();

        services.add(transient_factory(move |_| {
            ::di::Ref::new(ConfigChangeTokenSource::<B, T>::new(name.clone(), reloadable.clone()))
                as ::di::Ref<dyn OptionsChangeTokenSource<T>>
        }));
    } else {
        services.add(transient_factory(move |_| {
            ::di::Ref::new(ConfigChangeTokenSource::<_, T>::new(name.clone(), NeverReloads))
                as ::di::Ref<dyn OptionsChangeTokenSource<T>>
        }));
    }
}

impl<C> ConfigExt<C> for ServiceCollection
where
    C: Value + Binder + Reloadable + Clone + 'static,
{
    fn apply_config<T>(&mut self, configuration: C) -> OptionsBuilder<'_, T>
    where
        T: Value + Default + DeserializeOwned + 'static,
    {
        add_change_token_source::<_, T>(self, None, &configuration);
        self.add_options()
            .configure(move |options: &mut T| configuration.bind_unchecked(options))
    }

    fn apply_config_at<T>(&mut self, configuration: C, key: impl AsRef<str>) -> OptionsBuilder<'_, T>
    where
        T: Value + Default + DeserializeOwned + 'static,
    {
        let key = key.as_ref().to_owned();

        add_change_token_source::<_, T>(self, Some(key.clone()), &configuration);

        self.add_named_options(&key)
            .configure(move |options: &mut T| configuration.bind_at_unchecked(&key, options))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Options, OptionsMonitor, OptionsSnapshot};
    use ::config::ReloadableConfiguration;
    use ::di::ServiceCollection;
    use cfg_if::cfg_if;
    use serde::Deserialize;
    use serde_json::json;
    use std::convert::TryInto;
    use std::fs::File;
    use std::io::Write;
    use std::sync::{Arc, Condvar, Mutex};
    use std::time::{Duration, Instant};
    use tempfile::tempdir;

    #[derive(Default, Deserialize)]
    struct TestOptions {
        enabled: bool,
    }

    #[test]
    fn apply_config_should_bind_configuration_to_options() {
        // arrange
        let config = ::config::builder()
            .add_in_memory(&[("Enabled", "true")])
            .build()
            .unwrap();
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
        let config = ::config::builder()
            .add_in_memory(&[("Test:Enabled", "true")])
            .build()
            .unwrap();
        let provider = ServiceCollection::new()
            .apply_config::<TestOptions>(config.section("Test").to_owned())
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
        let config = ::config::builder()
            .add_in_memory(&[("Test:Enabled", "true")])
            .build()
            .unwrap();
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
        let dir = tempdir().unwrap();
        let path = dir.path().join("options.json");
        let mut json = json!({"enabled": true});
        let mut file = File::create(&path).unwrap();

        file.write_all(json.to_string().as_bytes()).unwrap();
        drop(file);

        let config: ReloadableConfiguration = ::config::builder()
            .add_json_file(&path.is().reloadable())
            .try_into()
            .unwrap();
        let provider = ServiceCollection::new()
            .apply_config::<TestOptions>(config)
            .build_provider()
            .unwrap();
        let monitor = provider.get_required::<dyn OptionsMonitor<TestOptions>>();
        let original = monitor.current_value();
        let state = Arc::new((Mutex::new(false), Condvar::new()));
        let state_clone = state.clone();
        let _sub = monitor.on_change(Box::new(move |_, _| {
            let (reloaded, event) = &*state_clone;
            *reloaded.lock().unwrap() = true;
            event.notify_one();
        }));

        json = json!({"enabled": false});
        file = File::create(&path).unwrap();
        file.write_all(json.to_string().as_bytes()).unwrap();
        drop(file);

        let (mutex, event) = &*state;
        let deadline = Instant::now();

        loop {
            cfg_if! {
                if #[cfg(not(feature = "async"))] {
                    // the sync path is not guaranteed to be Send or Sync so it cannot automatically updated in the
                    // background. the change happens in the background, but it is not realized until the next time
                    // the value is polled. poll the monitor to force realization of the change
                    let _ = monitor.current_value();
                }
            }

            let mut reloaded = mutex.lock().unwrap();

            if *reloaded {
                break;
            }

            if deadline.elapsed() > Duration::from_secs(3) {
                panic!("timed out waiting for configuration reload");
            }

            reloaded = event.wait_timeout(reloaded, Duration::from_millis(250)).unwrap().0;

            if *reloaded {
                break;
            }
        }

        // act
        let current = monitor.current_value();

        // assert
        assert_eq!(original.enabled, true);
        assert_eq!(current.enabled, false);
    }
}

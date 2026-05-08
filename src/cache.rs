use crate::{validation::Error, Ref, Value};
use std::collections::HashMap;
use std::sync::Mutex;

/// Defines the behavior of an options monitor cache.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait MonitorCache<T: Value> {
    /// Gets or adds options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    /// * `create` - The function used to create options when added
    fn get_or_add(&self, name: &str, create: &dyn Fn(&str) -> Result<T, Error>) -> Result<Ref<T>, Error>;

    /// Attempts to add options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    /// * `options` - The options to add
    fn try_add(&self, name: &str, options: T) -> bool;

    /// Attempts to remove options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    fn try_remove(&self, name: &str) -> bool;

    /// Clears all options from the cache.
    fn clear(&self);
}

/// Represents a cache for configured options.
pub struct Cache<T>(Mutex<HashMap<String, Ref<T>>>);

impl<T> Default for Cache<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Value> MonitorCache<T> for Cache<T> {
    fn get_or_add(&self, name: &str, create: &dyn Fn(&str) -> Result<T, Error>) -> Result<Ref<T>, Error> {
        let mut cache = self.0.lock().unwrap();

        if let Some(options) = cache.get(name) {
            return Ok(options.clone());
        }

        let options = Ref::new(create(name)?);

        cache.insert(name.to_owned(), options.clone());

        Ok(options)
    }

    fn try_add(&self, name: &str, options: T) -> bool {
        let mut cache = self.0.lock().unwrap();

        if cache.contains_key(name) {
            false
        } else {
            cache.insert(name.into(), Ref::new(options));
            true
        }
    }

    #[inline]
    fn try_remove(&self, name: &str) -> bool {
        self.0.lock().unwrap().remove(name).is_some()
    }

    #[inline]
    fn clear(&self) {
        self.0.lock().unwrap().clear()
    }
}

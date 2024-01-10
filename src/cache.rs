use crate::{Ref, Value};
use std::collections::HashMap;
use std::sync::Mutex;

/// Defines the behavior of an [`Options`](crate::Options) monitor cache.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait OptionsMonitorCache<T: Value> {
    /// Gets or adds options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    /// * `create_options` - The function used to create options when added
    fn get_or_add(&self, name: Option<&str>, create_options: &dyn Fn(Option<&str>) -> T) -> Ref<T>;

    /// Attempts to add options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    /// * `options` - The options to add
    fn try_add(&self, name: Option<&str>, options: T) -> bool;

    /// Attempts to remove options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    fn try_remove(&self, name: Option<&str>) -> bool;

    /// Clears all options from the cache.
    fn clear(&self);
}

/// Represents a cache for configured options.
pub struct OptionsCache<T> {
    cache: Mutex<HashMap<String, Ref<T>>>,
}

impl<T> Default for OptionsCache<T> {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

unsafe impl<T: Send + Sync> Send for OptionsCache<T> {}
unsafe impl<T: Send + Sync> Sync for OptionsCache<T> {}

impl<T: Value> OptionsMonitorCache<T> for OptionsCache<T> {
    fn get_or_add(&self, name: Option<&str>, create_options: &dyn Fn(Option<&str>) -> T) -> Ref<T> {
        let key = name.unwrap_or_default().to_string();
        self.cache
            .lock()
            .unwrap()
            .entry(key)
            .or_insert_with(|| Ref::new(create_options(name)))
            .clone()
    }

    fn try_add(&self, name: Option<&str>, options: T) -> bool {
        let key = name.unwrap_or_default();
        let mut cache = self.cache.lock().unwrap();

        if cache.contains_key(key) {
            false
        } else {
            cache.insert(key.to_owned(), Ref::new(options));
            true
        }
    }

    fn try_remove(&self, name: Option<&str>) -> bool {
        let key = name.unwrap_or_default();
        self.cache.lock().unwrap().remove(key).is_some()
    }

    fn clear(&self) {
        self.cache.lock().unwrap().clear()
    }
}

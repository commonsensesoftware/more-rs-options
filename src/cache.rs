use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::Mutex;

/// Defines the behavior of an options monitor cache.
pub trait OptionsMonitorCache<T> {
    /// Gets or adds options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    /// * `create_options` - The function used to create options when added
    fn get_or_add(&self, name: Option<&str>, create_options: &dyn Fn(Option<&str>) -> T) -> &T;

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
    // this is to support 'get_or_add' without requiring a mutable reference.
    // '&mut' is too restrictive and RefCell won't help here because we need
    // a borrowed reference at the end
    sync: Mutex<String>,
    cache: UnsafeCell<HashMap<String, T>>,
}

impl<T> Default for OptionsCache<T> {
    fn default() -> Self {
        Self {
            sync: Default::default(),
            cache: Default::default(),
        }
    }
}

impl<T> OptionsMonitorCache<T> for OptionsCache<T> {
    fn get_or_add(&self, name: Option<&str>, create_options: &dyn Fn(Option<&str>) -> T) -> &T {
        let key = name.unwrap_or_default().to_string();
        let _lock = self.sync.lock().unwrap();

        unsafe {
            let cache: &mut HashMap<String, T> = &mut *self.cache.get();
            cache.entry(key).or_insert_with(|| create_options(name))
        }
    }

    fn try_add(&self, name: Option<&str>, options: T) -> bool {
        let key = name.unwrap_or_default();
        let _lock = self.sync.lock().unwrap();

        unsafe {
            let cache: &mut HashMap<String, T> = &mut *self.cache.get();

            if cache.contains_key(key) {
                false
            } else {
                cache.insert(key.to_owned(), options);
                true
            }
        }
    }

    fn try_remove(&self, name: Option<&str>) -> bool {
        let key = name.unwrap_or_default();
        let _lock = self.sync.lock().unwrap();

        unsafe {
            let cache: &mut HashMap<String, T> = &mut *self.cache.get();
            cache.remove(key).is_some()
        }
    }

    fn clear(&self) {
        let _lock = self.sync.lock().unwrap();
        unsafe {
            let cache: &mut HashMap<String, T> = &mut *self.cache.get();
            cache.clear();
        }
    }
}

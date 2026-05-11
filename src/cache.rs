use crate::{validation::Error, Ref, Value};
use std::collections::HashMap;
use std::sync::RwLock;

/// Represents a monitored cache of configured options.
pub struct Cache<T>(RwLock<HashMap<String, Ref<T>>>);

impl<T> Default for Cache<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Value> Cache<T> {
    /// Initializes a new options [Cache].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets or adds options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    /// * `create` - The function used to create options when added
    pub fn get_or_add(&self, name: &str, create: &dyn Fn(&str) -> Result<T, Error>) -> Result<Ref<T>, Error> {
        let cache = self.0.read().unwrap();

        if let Some(options) = cache.get(name) {
            return Ok(options.clone());
        }

        drop(cache);

        let options = Ref::new(create(name)?);
        let mut cache = self.0.write().unwrap();

        if let Some(options) = cache.get(name) {
            Ok(options.clone())
        } else {
            cache.insert(name.to_owned(), options.clone());
            Ok(options)
        }
    }

    /// Attempts to add options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    /// * `options` - The options to add
    ///
    /// # Remarks
    ///
    /// Returns `true` if the options were added; otherwise, `false`. Options are not added if they already exist.
    #[must_use]
    pub fn try_add(&self, name: &str, options: T) -> bool {
        let mut cache = self.0.write().unwrap();

        if cache.contains_key(name) {
            false
        } else {
            cache.insert(name.into(), Ref::new(options));
            true
        }
    }

    /// Removes options with the specified name, if any.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options
    ///
    /// # Remarks
    ///
    /// Returns `true` if any options were removed; otherwise, `false`.
    pub fn remove(&self, name: &str) -> bool {
        self.0.write().unwrap().remove(name).is_some()
    }

    /// Clears all options from the cache.
    pub fn clear(&self) {
        self.0.write().unwrap().clear()
    }
}

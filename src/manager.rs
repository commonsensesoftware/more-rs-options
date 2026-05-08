use crate::{validation::Error, Cache, Factory, MonitorCache, Ref, Snapshot, Value};

/// Represents an object that manages [options](Options) and [option snapshots](OptionsSnapshot).
pub struct Manager<T: Value> {
    factory: Ref<dyn Factory<T>>,
    cache: Cache<T>,
}

impl<T: Value> Manager<T> {
    /// Initializes a new options manager.
    ///
    /// # Arguments
    ///
    /// * `factory` - The [factory](OptionsFactory) used to create new options
    #[inline]
    pub fn new(factory: Ref<dyn Factory<T>>) -> Self {
        Self {
            factory,
            cache: Default::default(),
        }
    }
}

impl<T: Value> Snapshot<T> for Manager<T> {
    fn get_named(&self, name: &str) -> Result<Ref<T>, Error> {
        self.cache.get_or_add(name, &|n| self.factory.create(n))
    }
}

use crate::{
    Options, OptionsCache, OptionsFactory, OptionsMonitorCache, OptionsSnapshot, Ref, Value,
};

/// Represents an object that manages [`Options`](crate::Options) and [option snapshots](crate::OptionsSnapshot).
pub struct OptionsManager<T: Value> {
    factory: Ref<dyn OptionsFactory<T>>,
    cache: OptionsCache<T>,
}

impl<T: Value> OptionsManager<T> {
    /// Initializes a new options manager.
    ///
    /// # Arguments
    ///
    /// * `factory` - The [factory](crate::OptionsFactory) used to create new options.
    #[inline]
    pub fn new(factory: Ref<dyn OptionsFactory<T>>) -> Self {
        Self {
            factory,
            cache: Default::default(),
        }
    }
}

impl<T: Value> Options<T> for OptionsManager<T> {
    #[inline]
    fn value(&self) -> Ref<T> {
        self.get(None)
    }
}

impl<T: Value> OptionsSnapshot<T> for OptionsManager<T> {
    #[inline]
    fn get(&self, name: Option<&str>) -> Ref<T> {
        self.cache
            .get_or_add(name, &|n| self.factory.create(n).unwrap())
    }
}

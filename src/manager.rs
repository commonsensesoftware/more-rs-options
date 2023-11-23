use crate::{Options, OptionsCache, OptionsFactory, OptionsMonitorCache, OptionsSnapshot, Ref};

/// Represents an object that manages [`Options`](crate::Options) and [option snapshots](crate::OptionsSnapshot).
pub struct OptionsManager<T> {
    factory: Ref<dyn OptionsFactory<T>>,
    cache: OptionsCache<T>,
}

impl<T> OptionsManager<T> {
    /// Initializes a new options manager.
    ///
    /// # Arguments
    ///
    /// * `factory` - The [factory](crate::OptionsFactory) used to create new options.
    pub fn new(factory: Ref<dyn OptionsFactory<T>>) -> Self {
        Self {
            factory,
            cache: Default::default(),
        }
    }
}

impl<T> Options<T> for OptionsManager<T> {
    fn value(&self) -> Ref<T> {
        self.get(None)
    }
}

impl<T> OptionsSnapshot<T> for OptionsManager<T> {
    fn get(&self, name: Option<&str>) -> Ref<T> {
        self.cache
            .get_or_add(name, &|n| self.factory.create(n).unwrap())
    }
}

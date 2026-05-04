use crate::{Options, OptionsCache, OptionsFactory, OptionsMonitorCache, OptionsSnapshot, Ref, Value};

/// Represents an object that manages [options](Options) and [option snapshots](OptionsSnapshot).
pub struct OptionsManager<T: Value> {
    factory: Ref<dyn OptionsFactory<T>>,
    cache: OptionsCache<T>,
}

impl<T: Value> OptionsManager<T> {
    /// Initializes a new options manager.
    ///
    /// # Arguments
    ///
    /// * `factory` - The [factory](OptionsFactory) used to create new options
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
    fn get(&self, name: Option<&str>) -> Ref<T> {
        self.cache.get_or_add(name, &|n| match self.factory.create(n) {
            Ok(options) => options,
            Err(error) => panic!("{}", error),
        })
    }
}

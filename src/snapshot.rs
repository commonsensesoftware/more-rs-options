use crate::{Ref, Value, validation::Error};

/// Defines the behavior for a snapshot of configuration options.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait Snapshot<T: Value> {
    /// Gets the default, unnamed configuration options.
    fn get(&self) -> Result<Ref<T>, Error> {
        self.get_named("")
    }

    /// Gets the default, unnamed configuration options.
    /// 
    /// # Remarks
    ///
    /// This function panics if the configuration options could not be successfully retrieved.
    fn get_unchecked(&self) -> Ref<T> {
        match self.get_named("") {
            Ok(value) => value,
            Err(error) => panic!("{}", error),
        }
    }
    
    /// Gets the configuration options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to retrieve
    fn get_named(&self, name: &str) -> Result<Ref<T>, Error>;

    /// Gets the configuration options with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to retrieve
    ///
    /// # Remarks
    ///
    /// This function panics if the configuration options could not be successfully retrieved.
    fn get_named_unchecked(&self, name: &str) -> Ref<T> {
        match self.get_named(name) {
            Ok(value) => value,
            Err(error) => panic!("[{name}] {}", error),
        }
    }
}

use crate::{Ref, Value};

/// Defines the behavior for a snapshot of configuration [`Options`](crate::Options).
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait OptionsSnapshot<T: Value> {
    /// Gets the configuration options with the specified name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The optional name of the options to retrieve
    fn get(&self, name: Option<&str>) -> Ref<T>;
}
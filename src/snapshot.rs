use crate::Ref;

/// Defines the behavior for a snapshot of configuration [`Options`](crate::Options).
pub trait OptionsSnapshot<T> {
    /// Gets the configuration options with the specified name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The optional name of the options to retrieve
    fn get(&self, name: Option<&str>) -> Ref<T>;
}
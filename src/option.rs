use crate::Ref;

/// Defines the behavior to retrieve configured options.
pub trait Options<T> {
    /// Gets the configured value.
    fn value(&self) -> Ref<T>;
}

/// Creates a wrapper around a value to return itself as [`Options`](Options).
///
/// # Arguments
///
/// * `options` - The options value to wrap.
pub fn create<T>(options: T) -> impl Options<T> {
    OptionsWrapper(Ref::new(options))
}

struct OptionsWrapper<T>(Ref<T>);

impl<T> Options<T> for OptionsWrapper<T> {
    fn value(&self) -> Ref<T> {
        self.0.clone()
    }
}

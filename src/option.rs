/// Defines the behavior to retrieve configured options.
pub trait Options<T> {
    /// Gets the configured value.
    fn value(&self) -> &T;
}

/// Creates a wrapper around a value to return itself as [options](trait.Options.html).
///
/// # Arguments
///
/// * `options` - The options value to wrap.
pub fn create<T>(options: T) -> impl Options<T> {
    OptionsWrapper(options)
}

struct OptionsWrapper<T>(T);

impl<T> Options<T> for OptionsWrapper<T> {
    fn value(&self) -> &T {
        &self.0
    }
}

use crate::{Ref, Value};

/// Defines the behavior to retrieve configured options.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait Options<T: Value> {
    /// Gets the configured value.
    fn value(&self) -> Ref<T>;
}

/// Creates a wrapper around a value to return itself as [`Options`](Options).
///
/// # Arguments
///
/// * `options` - The options value to wrap.
pub fn create<T: Value>(options: T) -> impl Options<T> {
    OptionsWrapper(Ref::new(options))
}

struct OptionsWrapper<T: Value>(Ref<T>);

impl<T: Value> Options<T> for OptionsWrapper<T> {
    fn value(&self) -> Ref<T> {
        self.0.clone()
    }
}

unsafe impl<T: Send + Sync> Send for OptionsWrapper<T> {}
unsafe impl<T: Send + Sync> Sync for OptionsWrapper<T> {}
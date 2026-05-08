use super::Result;
use cfg_if::cfg_if;

/// Validates options.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait Validate<T> {
    /// Validates named options or all options if no name is specified.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to validate
    /// * `options` - The options to validate
    fn run(&self, name: &str, options: &T) -> Result;
}

macro_rules! validate {
    ($($bounds:tt)+) => {
        impl<F: Fn(&str, &T) -> Result + $($bounds)+, T> Validate<T> for F {
            fn run(&self, name: &str, options: &T) -> Result {
                (self)(name, options)
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "async")] {
        validate!(Sized + Send + Sync);
    } else {
        validate!(Sized);
    }
}

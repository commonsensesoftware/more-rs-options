use cfg_if::cfg_if;

/// Configures options.
///
/// # Remarks
///
/// These are all run first.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait Configure<T> {
    /// Runs the configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to configure
    /// * `options` - The options to configure
    fn run(&self, name: &str, options: &mut T);
}

/// Configures options.
///
/// # Remarks
///
/// These are all run last.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait PostConfigure<T> {
    /// Runs the configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to configure
    /// * `options` - The options to configure
    fn run(&self, name: &str, options: &mut T);
}

macro_rules! configure {
    ($($bounds:tt)+) => {
        impl<F: Fn(&str, &mut T) + $($bounds)+, T> Configure<T> for F {
            fn run(&self, name: &str, options: &mut T) {
                (self)(name, options)
            }
        }

        impl<F: Fn(&str, &mut T) + $($bounds)+, T> PostConfigure<T> for F {
            fn run(&self, name: &str, options: &mut T) {
                (self)(name, options)
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "async")] {
        configure!(Sized + Send + Sync);
    } else {
        configure!(Sized);
    }
}

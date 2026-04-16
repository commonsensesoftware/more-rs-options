use crate::Value;
use cfg_if::cfg_if;
use std::marker::PhantomData;

/// Defines the behavior of something that configures [`Options`](crate::Options).
///
/// # Remarks
///
/// These are all run first
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait ConfigureOptions<T> {
    /// Configures the corresponding options.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to configure
    /// * `options` - The options to configure
    fn configure(&self, name: Option<&str>, options: &mut T);
}

/// Defines the behavior of something that configures [`Options`](crate::Options).
///
/// # Remarks
///
/// These are all run last
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait PostConfigureOptions<T> {
    /// Configures the corresponding options.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to configure
    /// * `options` - The options to configure
    fn post_configure(&self, name: Option<&str>, options: &mut T);
}

macro_rules! config_impl {
    (($($bounds:tt)+)) => {
        /// Creates and returns [options configuration](ConfigureOptions) for the specified action.
        ///
        /// # Arguments
        ///
        /// * `action` - The configuration action
        #[inline]
        pub fn configure<T, F>(action: F) -> impl ConfigureOptions<T>
        where
            T: Value,
            F: Fn(Option<&str>, &mut T) + $($bounds)+,
        {
            _ConfigureOptions {
                action,
                _marker: PhantomData,
            }
        }

        /// Creates and returns [options post-configuration](PostConfigureOptions) for the specified action.
        ///
        /// # Arguments
        ///
        /// * `action` - The post configuration action
        #[inline]
        pub fn post_configure<T, F>(action: F) -> impl PostConfigureOptions<T>
        where
            T: Value,
            F: Fn(Option<&str>, &mut T) + $($bounds)+,
        {
            _ConfigureOptions {
                action,
                _marker: PhantomData,
            }
        }

        struct _ConfigureOptions<TOptions, TAction> {
            action: TAction,
            _marker: PhantomData<TOptions>,
        }

        impl<TOptions, TAction> ConfigureOptions<TOptions> for _ConfigureOptions<TOptions, TAction>
        where
            TOptions: Value,
            TAction: Fn(Option<&str>, &mut TOptions) + $($bounds)+,
        {
            #[inline]
            fn configure(&self, name: Option<&str>, options: &mut TOptions) {
                (self.action)(name, options)
            }
        }

        impl<TOptions, TAction> PostConfigureOptions<TOptions>
            for _ConfigureOptions<TOptions, TAction>
        where
            TOptions: Value,
            TAction: Fn(Option<&str>, &mut TOptions) + $($bounds)+,
        {
            #[inline]

            fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
                (self.action)(name, options)
            }
        }
    };
}

cfg_if! {
    if #[cfg(feature = "async")] {
        config_impl!((Send + Sync + 'static));
    } else {
        config_impl!(('static));
    }
}

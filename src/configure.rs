use std::marker::PhantomData;

/// Defines the behavior of something that configures [`Options`](crate::Options).
///
/// # Remarks
///
/// These are all run first
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
pub trait PostConfigureOptions<T> {
    /// Configures the corresponding options.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to configure
    /// * `options` - The options to configure
    fn post_configure(&self, name: Option<&str>, options: &mut T);
}

/// Creates and returns [options configuration](ConfigureOptions) for the specified action.
///
/// # Arguments
///
/// * `action` - The configuration action
pub fn configure<T, F>(action: F) -> impl ConfigureOptions<T>
where
    F: Fn(Option<&str>, &mut T),
{
    _ConfigureOptions::new(action)
}

/// Creates and returns [options post-configuration](PostConfigureOptions) for the specified action.
///
/// # Arguments
///
/// * `action` - The post configuration action
pub fn post_configure<T, F>(action: F) -> impl PostConfigureOptions<T>
where
    F: Fn(Option<&str>, &mut T),
{
    _ConfigureOptions::new(action)
}

struct _ConfigureOptions<TOptions, TAction>
where
    TAction: Fn(Option<&str>, &mut TOptions),
{
    action: TAction,
    _marker: PhantomData<TOptions>,
}

impl<TOptions, TAction> _ConfigureOptions<TOptions, TAction>
where
    TAction: Fn(Option<&str>, &mut TOptions),
{
    fn new(action: TAction) -> Self {
        Self {
            action,
            _marker: PhantomData,
        }
    }
}

impl<TOptions, TAction> ConfigureOptions<TOptions> for _ConfigureOptions<TOptions, TAction>
where
    TAction: Fn(Option<&str>, &mut TOptions),
{
    fn configure(&self, name: Option<&str>, options: &mut TOptions) {
        (self.action)(name, options)
    }
}

impl<TOptions, TAction> PostConfigureOptions<TOptions> for _ConfigureOptions<TOptions, TAction>
where
    TAction: Fn(Option<&str>, &mut TOptions),
{
    fn post_configure(&self, name: Option<&str>, options: &mut TOptions) {
        (self.action)(name, options)
    }
}

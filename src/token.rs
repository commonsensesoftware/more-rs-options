use crate::Value;
use tokens::ChangeToken;

/// Used to fetch [`ChangeToken`](tokens::ChangeToken) used for tracking options changes.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait OptionsChangeTokenSource<T: Value> {
    /// Creates and returns a [`ChangeToken`](tokens::ChangeToken) which can be
    /// used to register a change notification callback.
    fn token(&self) -> Box<dyn ChangeToken>;

    /// Gets the name of the option instance being changed, if any.
    fn name(&self) -> Option<&str> {
        None
    }
}

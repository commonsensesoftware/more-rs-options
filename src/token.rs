use crate::Value;
use tokens::ChangeToken;

/// Used to fetch a [change token](ChangeToken) to track options changes.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait ChangeTokenSource<T: Value> {
    /// Creates and returns a [change token](ChangeToken) which can be used to register a change notification callback.
    fn token(&self) -> Box<dyn ChangeToken>;

    /// Gets the name of the option instance being changed, if any.
    fn name(&self) -> &str {
        ""
    }
}

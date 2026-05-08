mod error;
mod outcome;
mod validate;

pub use error::Error;
pub use outcome::Outcome;
pub use validate::Validate;

/// Represents a validation result.
pub type Result = std::result::Result<Outcome, Error>;

/// Indicates successful validation.
#[inline]
pub fn success() -> Result {
    Ok(Outcome::Succeeded)
}

/// Indicates validation was skipped.
#[inline]
pub fn skip() -> Result {
    Ok(Outcome::Skipped)
}

/// Indicates validation failed.
///
/// # Arguments
///
/// * `failure` - The validation failure message
#[inline]
pub fn fail(failure: impl AsRef<str>) -> Result {
    Err(Error::new(failure))
}

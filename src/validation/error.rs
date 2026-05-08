use std::error;
use std::fmt::{Display, Formatter, Result};

/// Represents the result of options validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    failures: Vec<String>,
}

impl Error {
    /// Creates a new validation [Error].
    ///
    /// # Arguments
    ///
    /// `failure` - The failure message
    #[inline]
    pub fn new<S: AsRef<str>>(failure: S) -> Self {
        Self::many([failure])
    }

    /// Creates a new validation [Error] for many reasons.
    ///
    /// # Arguments
    ///
    /// `failures` - The failure messages
    pub fn many<S, I>(failures: I) -> Self
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        Self {
            failures: failures.into_iter().map(|f| f.as_ref().to_owned()).collect(),
        }
    }

    /// Gets the full list of validation failures.
    #[inline]
    pub fn failures(&self) -> &[String] {
        &self.failures
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut failures = self.failures.iter();

        if let Some(failure) = failures.next() {
            failure.fmt(f)?;

            for failure in failures {
                f.write_str("; ")?;
                failure.fmt(f)?;
            }
        }

        Ok(())
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_return_message() {
        // arrange
        let result = Error::new("Failed");

        // act
        let message = result.to_string();

        // assert
        assert_eq!(message, "Failed");
    }

    #[test]
    fn many_should_return_joined_message() {
        // arrange
        let failures = ["Failure 1", "Failure 2"];
        let result = Error::many(failures);

        // act
        let message = result.to_string();

        // assert
        assert_eq!(message, "Failure 1; Failure 2");
    }

    #[test]
    fn many_should_return_failures() {
        // arrange
        let expected = ["Failure 1", "Failure 2"];
        let result = Error::many(&expected);

        // act
        let failures = result.failures();

        // assert
        assert_eq!(failures, &expected[..]);
    }
}

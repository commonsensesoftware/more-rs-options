use std::fmt::{Display, Formatter, Result as FormatResult};

/// Represents the result of [`Options`](crate::Options) validation.
#[derive(Clone, Debug)]
pub struct ValidateOptionsResult {
    succeeded: bool,
    skipped: bool,
    failed: bool,
    failures: Vec<String>,
}

impl ValidateOptionsResult {
    /// Gets a value indicating whether the validation was successful.
    pub fn succeeded(&self) -> bool {
        self.succeeded
    }

    /// Gets a value indicating whether the validation was skipped.
    pub fn skipped(&self) -> bool {
        self.skipped
    }

    /// Gets a value indicating whether the validation failed.
    pub fn failed(&self) -> bool {
        self.failed
    }

    /// Gets the validation failure description.
    pub fn failure_message(&self) -> String {
        if self.failures.is_empty() {
            String::new()
        } else {
            self.failures.join("; ")
        }
    }

    /// Gets the full list of validation failures.
    pub fn failures(&self) -> &[String] {
        &self.failures
    }

    /// Creates a result when validation was skipped due to not matching.
    pub fn skip() -> Self {
        Self {
            succeeded: false,
            skipped: true,
            failed: false,
            failures: Vec::with_capacity(0),
        }
    }

    /// Creates a result when validation was successful.
    pub fn success() -> Self {
        Self {
            succeeded: true,
            skipped: false,
            failed: false,
            failures: Vec::with_capacity(0),
        }
    }

    /// Creates a result when validation failed.
    ///
    /// # Arguments
    ///
    /// `failure` - The failure message
    pub fn fail<S: AsRef<str>>(failure: S) -> Self {
        Self::fail_many([failure].iter())
    }

    /// Creates a result when validation failed for many reasons.
    pub fn fail_many<S, I>(failures: I) -> Self
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        Self {
            succeeded: false,
            skipped: false,
            failed: true,
            failures: failures.map(|f| f.as_ref().to_owned()).collect(),
        }
    }
}

impl Display for ValidateOptionsResult {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        formatter.write_str(&self.failure_message())
    }
}

/// Defines the behavior of an object that validates configuration options.
pub trait ValidateOptions<T> {
    /// Validates named options or all options if no name is specified.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the options to validate
    /// * `options` - The options to validate
    fn validate(&self, name: Option<&str>, options: &T) -> ValidateOptionsResult;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn success_should_indicate_succeed() {
        // arrange
        let result = ValidateOptionsResult::success();

        // act
        let succeeded = result.succeeded();

        // assert
        assert!(succeeded);
    }

    #[test]
    fn skip_should_indicate_skipped() {
        // arrange
        let result = ValidateOptionsResult::skip();

        // act
        let skipped = result.skipped();

        // assert
        assert!(skipped);
    }

    #[test]
    fn fail_should_return_failed() {
        // arrange
        let result = ValidateOptionsResult::fail("");

        // act
        let failed = result.failed();

        // assert
        assert!(failed);
    }

    #[test]
    fn fail_should_return_message() {
        // arrange
        let result = ValidateOptionsResult::fail("Failed");

        // act
        let message = result.failure_message();

        // assert
        assert_eq!(&message, "Failed");
    }

    #[test]
    fn fail_many_should_return_joined_message() {
        // arrange
        let failures = ["Failure 1", "Failure 2"];
        let result = ValidateOptionsResult::fail_many(failures.iter());

        // act
        let message = result.failure_message();

        // assert
        assert_eq!(&message, "Failure 1; Failure 2");
    }

    #[test]
    fn fail_many_should_return_failures() {
        // arrange
        let expected = ["Failure 1", "Failure 2"];
        let result = ValidateOptionsResult::fail_many(expected.iter());

        // act
        let failures = result.failures();

        // assert
        assert_eq!(failures, &expected[..]);
    }

    #[test]
    fn to_string_should_return_message() {
        // arrange
        let result = ValidateOptionsResult::fail("Failed");
        let message = result.failure_message();

        // act
        let string = result.to_string();

        // assert
        assert_eq!(string, message);
    }
}

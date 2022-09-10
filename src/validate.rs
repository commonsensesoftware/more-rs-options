use std::fmt::{Display, Formatter, Result as FormatResult};

/// Represents the result of options validation.
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
    pub fn fail(failure_message: impl AsRef<str>) -> Self {
        Self::fail_many(vec![failure_message.as_ref().to_owned()])
    }

    /// Creates a result when validation failed for many reasons.
    pub fn fail_many(failures: Vec<String>) -> Self {
        Self {
            succeeded: false,
            skipped: false,
            failed: true,
            failures,
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
        let failures = vec!["Failure 1".to_owned(), "Failure 2".to_owned()];
        let result = ValidateOptionsResult::fail_many(failures);
        
        // act
        let message = result.failure_message();

        // assert
        assert_eq!(&message, "Failure 1; Failure 2");
    }

    #[test]
    fn fail_many_should_return_failures() {
        // arrange
        let expected: Vec<String> = vec!["Failure 1", "Failure 2"]
            .into_iter()
            .map(String::from)
            .collect();
        let result = ValidateOptionsResult::fail_many(expected.clone());

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

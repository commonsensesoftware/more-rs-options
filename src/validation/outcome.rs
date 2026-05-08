/// Indicates the validation outcome.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Outcome {
    /// Indicates validation succeeded.
    #[default]
    Succeeded,

    /// Indicates validation was skipped.
    Skipped,
}

impl Outcome {
    /// Determines whether the validation succeeded.
    #[inline]
    pub fn succeeded(&self) -> bool {
        matches!(self, Self::Succeeded)
    }

    /// Determines whether the validation was skipped.
    #[inline]
    pub fn skipped(&self) -> bool {
        matches!(self, Self::Skipped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_should_indicate_succeeded() {
        // arrange
        let result = Outcome::Succeeded;

        // act
        let succeeded = result.succeeded();

        // assert
        assert!(succeeded);
    }

    #[test]
    fn skip_should_indicate_skipped() {
        // arrange
        let result = Outcome::Skipped;

        // act
        let skipped = result.skipped();

        // assert
        assert!(skipped);
    }
}

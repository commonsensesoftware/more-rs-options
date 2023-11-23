use crate::*;

/// Defines the behavior of an object that creates configuration [`Options`](crate::Options).
pub trait OptionsFactory<T> {
    /// Creates and returns new configuration options.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the configuration options to create
    fn create(&self, name: Option<&str>) -> Result<T, ValidateOptionsResult>;
}

/// Represents the default factory used to create configuration [`Options`](crate::Options).
#[derive(Default)]
pub struct DefaultOptionsFactory<T: Default> {
    configurations: Vec<Ref<dyn ConfigureOptions<T>>>,
    post_configurations: Vec<Ref<dyn PostConfigureOptions<T>>>,
    validations: Vec<Ref<dyn ValidateOptions<T>>>,
}

impl<T: Default> DefaultOptionsFactory<T> {
    /// Initializes a new options factory.
    ///
    /// # Arguments
    ///
    /// * `configurations` - The configurations used to [configure options](crate::ConfigureOptions).
    /// * `post_configurations` - The configurations used to [post-configure options](crate::PostConfigureOptions).
    /// * `validations` - The validations used to [validate options](crate::ValidateOptions).
    pub fn new(
        configurations: Vec<Ref<dyn ConfigureOptions<T>>>,
        post_configurations: Vec<Ref<dyn PostConfigureOptions<T>>>,
        validations: Vec<Ref<dyn ValidateOptions<T>>>,
    ) -> Self {
        Self {
            configurations,
            post_configurations,
            validations,
        }
    }
}

impl<T: Default> OptionsFactory<T> for DefaultOptionsFactory<T> {
    fn create(&self, name: Option<&str>) -> Result<T, ValidateOptionsResult> {
        let mut options = Default::default();

        for configuration in &self.configurations {
            configuration.configure(name, &mut options);
        }

        for configuration in &self.post_configurations {
            configuration.post_configure(name, &mut options);
        }

        if !self.validations.is_empty() {
            let mut failures = Vec::new();

            for validation in &self.validations {
                let result = validation.validate(name, &options);

                if result.failed() {
                    failures.extend_from_slice(result.failures())
                }
            }

            if !failures.is_empty() {
                return Err(ValidateOptionsResult::fail_many(failures.iter()));
            }
        }

        Ok(options)
    }
}
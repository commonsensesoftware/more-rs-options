use crate::{
    validation::{Error, Validate},
    Configure, PostConfigure, Ref, Value,
};

/// Defines the behavior of a configuration options factory.
#[cfg_attr(feature = "async", maybe_impl::traits(Send, Sync))]
pub trait Factory<T: Value> {
    /// Creates and returns new configuration options.
    ///
    /// # Arguments
    ///
    /// * `name` - The optional name of the configuration options to create
    fn create(&self, name: &str) -> Result<T, Error>;
}

/// Represents the default factory used to create configuration options.
#[derive(Default)]
pub struct DefaultFactory<T: Value + Default> {
    configurations: Vec<Ref<dyn Configure<T>>>,
    post_configurations: Vec<Ref<dyn PostConfigure<T>>>,
    validations: Vec<Ref<dyn Validate<T>>>,
}

impl<T: Value + Default> DefaultFactory<T> {
    /// Initializes a new options factory.
    ///
    /// # Arguments
    ///
    /// * `configurations` - The configurations used to [configure options](Configure)
    /// * `post_configurations` - The configurations used to [post-configure options](PostConfigure)
    /// * `validations` - The validations used to [validate options](Validate)
    pub fn new(
        configurations: Vec<Ref<dyn Configure<T>>>,
        post_configurations: Vec<Ref<dyn PostConfigure<T>>>,
        validations: Vec<Ref<dyn Validate<T>>>,
    ) -> Self {
        Self {
            configurations,
            post_configurations,
            validations,
        }
    }
}

impl<T: Value + Default> Factory<T> for DefaultFactory<T> {
    fn create(&self, name: &str) -> Result<T, Error> {
        let mut options = Default::default();

        for configuration in &self.configurations {
            configuration.run(name, &mut options);
        }

        for configuration in &self.post_configurations {
            configuration.run(name, &mut options);
        }

        let mut failures = Vec::new();

        for validation in &self.validations {
            if let Err(error) = validation.run(name, &options) {
                failures.extend_from_slice(error.failures());
            }
        }

        if failures.is_empty() {
            Ok(options)
        } else {
            Err(Error::many(failures))
        }
    }
}

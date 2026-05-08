mod configure;
mod validate;

use di::ServiceCollection;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Represents a builder used to configure options.
pub struct Builder<'a, T: 'static> {
    name: String,
    services: &'a mut ServiceCollection,
    _type: PhantomData<T>,
}

impl<'a, T: 'static> Builder<'a, T> {
    /// Initializes a new options builder.
    ///
    /// # Arguments
    ///
    /// * `services` - The associated [collection of services](di::ServiceCollection)
    /// * `name` - The optional name associated with the options
    ///
    /// # Remarks
    ///
    /// Names are matched using case-insensitive ASCII characters.
    #[inline]
    pub fn new(services: &'a mut ServiceCollection, name: &str) -> Self {
        Self {
            name: name.into(),
            services,
            _type: PhantomData,
        }
    }

    /// Gets the name of the options
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the associated [collection of services](di::ServiceCollection)    
    #[inline]
    pub fn services(&mut self) -> &mut ServiceCollection {
        self.services
    }
}

impl<'a, T> From<Builder<'a, T>> for &'a mut ServiceCollection {
    #[inline]
    fn from(builder: Builder<'a, T>) -> Self {
        builder.services
    }
}

impl<'a, T> Deref for Builder<'a, T> {
    type Target = ServiceCollection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.services
    }
}

impl<'a, T> DerefMut for Builder<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.services
    }
}

fn names_equal(name: &str, other_name: &str) -> bool {
    name.len() == other_name.len() && name.eq_ignore_ascii_case(other_name)
}

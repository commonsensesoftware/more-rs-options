#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Represents the type alias for an options reference.
#[cfg(not(any(feature = "di", feature = "async")))]
pub type Ref<T> = std::rc::Rc<T>;

/// Represents the type alias for an options reference.
#[cfg(all(not(feature = "di"), feature = "async"))]
pub type Ref<T> = std::sync::Arc<T>;

/// Represents the type alias for an options reference.
#[cfg(all(feature = "di", not(feature = "async")))]
pub type Ref<T> = di::Ref<T>;

/// Represents the type alias for an options reference.
#[cfg(all(feature = "di", feature = "async"))]
pub type Ref<T> = di::Ref<T>;

// trait aliases are unstable so define a custom
// marker that can bridge the gap
//
// REF: https://github.com/rust-lang/rust/issues/41517

#[cfg(not(feature = "async"))]
pub trait Value: Sized {}

#[cfg(not(feature = "async"))]
impl<T> Value for T {}

#[cfg(feature = "async")]
pub trait Value: Sized + Send + Sync {}

#[cfg(feature = "async")]
impl<T: Send + Sync> Value for T {}

mod cache;
mod configure;
mod factory;
mod manager;
mod monitor;
mod snapshot;
mod token;

/// Provides options validation.
pub mod validation;

/// Contains the library prelude.
#[cfg(any(feature = "di", feature = "cfg"))]
pub mod prelude;

pub use cache::{Cache, MonitorCache};
pub use configure::{Configure, PostConfigure};
pub use factory::{DefaultFactory, Factory};
pub use manager::Manager;
pub use monitor::{DefaultMonitor, Monitor, Subscription};
pub use snapshot::Snapshot;
pub use token::ChangeTokenSource;

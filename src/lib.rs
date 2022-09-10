#![doc = include_str!("../README.md")]

#[cfg(not(feature = "di"))]
pub(crate) type Ref<T> = std::rc::Rc<T>;

#[cfg(feature = "di")]
pub(crate) type Ref<T> = di::ServiceRef<T>;

mod cache;
mod configure;
mod factory;
mod manager;
mod monitor;
mod option;
mod snapshot;
mod token;
mod validate;

#[cfg(feature = "di")]
mod di_ext;

#[cfg(feature = "di")]
mod builder;

#[cfg(feature = "cfg")]
mod cfg_ext;

pub use cache::*;
pub use configure::*;
pub use factory::*;
pub use manager::*;
pub use monitor::*;
pub use option::*;
pub use snapshot::*;
pub use token::*;
pub use validate::*;

#[cfg(feature = "di")]
pub use builder::*;

/// Contains options extension methods.
#[cfg(any(feature = "di", feature = "cfg"))]
pub mod ext {
    use super::*;
    
    #[cfg(feature = "di")]
    pub use di_ext::*;

    #[cfg(feature = "cfg")]
    pub use cfg_ext::*;
}

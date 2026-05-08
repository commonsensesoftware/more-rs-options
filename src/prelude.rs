#[cfg(feature = "di")]
mod builder;

#[cfg(feature = "cfg")]
mod config;

#[cfg(feature = "di")]
mod di;

#[cfg(feature = "di")]
#[cfg_attr(docsrs, doc(cfg(feature = "di")))]
pub use builder::Builder;

#[cfg(feature = "cfg")]
#[cfg_attr(docsrs, doc(cfg(feature = "cfg")))]
pub use config::ConfigExt;

#[cfg(feature = "di")]
#[cfg_attr(docsrs, doc(cfg(feature = "di")))]
pub use di::OptionsExt;

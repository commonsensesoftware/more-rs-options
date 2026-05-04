#[cfg(feature = "di")]
mod di;

#[cfg(feature = "di")]
mod builder;

#[cfg(feature = "cfg")]
mod config;

#[cfg(feature = "di")]
#[cfg_attr(docsrs, doc(cfg(feature = "di")))]
pub use builder::OptionsBuilder;

#[cfg(feature = "di")]
#[cfg_attr(docsrs, doc(cfg(feature = "di")))]
pub use di::OptionsExt;

#[cfg(feature = "cfg")]
#[cfg_attr(docsrs, doc(cfg(feature = "cfg")))]
pub use config::ConfigExt;

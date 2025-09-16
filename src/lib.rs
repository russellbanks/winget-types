#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

#[cfg(feature = "std")]
pub use camino;
pub use icu_locale;
pub use sha2;
pub use shared::*;

pub mod installer;
pub mod locale;
mod shared;
pub mod utils;
pub mod version;

#[cfg(feature = "std")]
pub type Path = camino::Utf8PathBuf;

#[cfg(not(feature = "std"))]
pub type Path = alloc::string::String;

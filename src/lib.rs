#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;
mod manifest;
mod manifest_type;
mod manifest_version;
mod manifests;
mod package_identifier;
mod package_version;
pub mod url;
pub mod utils;
pub mod version;

#[cfg(feature = "std")]
pub use camino;
pub use icu_locale;
pub use manifest::Manifest;
pub use manifest_type::{ManifestType, ManifestTypeWithLocale};
pub use manifest_version::ManifestVersion;
pub use manifests::*;
pub use package_family_name;
pub use package_identifier::{PackageIdentifier, PackageIdentifierError};
pub use package_version::{PackageVersion, PackageVersionError};
pub use sha2;
pub use utils::{language_tag::LanguageTag, sha_256::Sha256String};
pub use version::Version;

#[cfg(feature = "std")]
pub type PathBuf = camino::Utf8PathBuf;

#[cfg(not(feature = "std"))]
pub type PathBuf = alloc::string::String;

#[cfg(feature = "std")]
pub type Path = camino::Utf8Path;

#[cfg(not(feature = "std"))]
pub type Path = str;

pub const DISALLOWED_CHARACTERS: [char; 9] = ['\\', '/', ':', '*', '?', '\"', '<', '>', '|'];

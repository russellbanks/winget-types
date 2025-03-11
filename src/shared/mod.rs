mod generic;
mod language_tag;
mod manifest;
mod manifest_type;
mod manifest_version;
mod package_identifier;
mod package_version;
mod sha_256;
pub mod url;
mod version;

pub use generic::GenericManifest;
pub use language_tag::LanguageTag;
pub use manifest::Manifest;
pub use manifest_type::{ManifestType, ManifestTypeWithLocale};
pub use manifest_version::ManifestVersion;
pub use package_identifier::{PackageIdentifier, PackageIdentifierError};
pub use package_version::{PackageVersion, PackageVersionError};
pub use sha_256::Sha256String;
pub use version::Version;

pub const DISALLOWED_CHARACTERS: [char; 9] = ['\\', '/', ':', '*', '?', '\"', '<', '>', '|'];

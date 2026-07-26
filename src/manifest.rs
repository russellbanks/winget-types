use crate::{ManifestType, ManifestVersion, PackageIdentifier, PackageVersion};

pub trait Manifest {
    const SCHEMA: &'static str;

    const TYPE: ManifestType;

    /// Returns the package identifier.
    fn package_identifier(&self) -> &PackageIdentifier;

    /// Returns the package version.
    fn package_version(&self) -> &PackageVersion;

    /// Returns the manifest version.
    fn manifest_version(&self) -> ManifestVersion;

    /// Updates the manifest version to the [latest](ManifestVersion::DEFAULT).
    fn update_manifest_version(&mut self);
}

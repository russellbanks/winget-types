//! Round-trip tests for complete manifests.
//!
//! Each fixture is written in the order and format that the crate serializes to, so
//! deserializing and serializing it again must give back the exact same text. Comparing text
//! (rather than values) also catches fields that are silently dropped because of a wrong name.

#![cfg(feature = "serde")]

use serde::{Serialize, de::DeserializeOwned};
use winget_types::{
    DefaultLocaleManifest, InstallerManifest, LocaleManifest, Manifest, ManifestType,
    VersionManifest, installer::Scope,
};

const VERSION: &str = include_str!("fixtures/version.yaml");
const DEFAULT_LOCALE: &str = include_str!("fixtures/default_locale.yaml");
const LOCALE: &str = include_str!("fixtures/locale.yaml");
const INSTALLER_MULTI: &str = include_str!("fixtures/installer_multi.yaml");
const INSTALLER_ZIP_PORTABLE: &str = include_str!("fixtures/installer_zip_portable.yaml");
const INSTALLER_MSIX: &str = include_str!("fixtures/installer_msix.yaml");

/// Deserializes a fixture, checks it serializes back to the same text, and returns it.
fn round_trip<T: Serialize + DeserializeOwned>(fixture: &str) -> T {
    // Fixtures may be checked out with CRLF line endings on Windows
    let fixture = fixture.replace("\r\n", "\n");

    let manifest = serde_yaml::from_str::<T>(&fixture).unwrap();
    let serialized = serde_yaml::to_string(&manifest).unwrap();

    assert_eq!(
        serialized, fixture,
        "serialized manifest differs from fixture"
    );

    manifest
}

#[test]
fn version_manifest() {
    let manifest = round_trip::<VersionManifest>(VERSION);

    assert_eq!(manifest.package_identifier().as_str(), "Contoso.App");
    assert_eq!(manifest.default_locale().to_string(), "en-US");
}

#[test]
fn default_locale_manifest() {
    let manifest = round_trip::<DefaultLocaleManifest>(DEFAULT_LOCALE);

    assert_eq!(manifest.manifest_type, ManifestType::DefaultLocale);
    assert_eq!(manifest.tags.len(), 2);
    assert!(manifest.description.is_some());
}

#[test]
fn locale_manifest() {
    let manifest = round_trip::<LocaleManifest>(LOCALE);

    assert_eq!(manifest.package_locale.to_string(), "de-DE");
    assert_eq!(manifest.manifest_type, ManifestType::Locale);
}

#[test]
fn multi_installer_manifest() {
    let manifest = round_trip::<InstallerManifest>(INSTALLER_MULTI);

    assert_eq!(manifest.installers.len(), 2);
    assert_eq!(manifest.expected_return_codes.len(), 2);
    assert_eq!(
        manifest
            .installers
            .iter()
            .map(|installer| installer.scope(&manifest))
            .collect::<Vec<_>>(),
        [Some(Scope::User), Some(Scope::Machine)]
    );
}

#[test]
fn zip_portable_installer_manifest() {
    let manifest = round_trip::<InstallerManifest>(INSTALLER_ZIP_PORTABLE);

    assert_eq!(manifest.dependencies.packages.len(), 1);
    assert!(manifest.archive_binaries_depend_on_path);
    assert!(
        manifest
            .installers
            .iter()
            .all(|installer| installer.nested_installer_files.len() == 1)
    );
}

#[test]
fn msix_installer_manifest() {
    let manifest = round_trip::<InstallerManifest>(INSTALLER_MSIX);

    assert!(manifest.package_family_name.is_some());
    assert!(manifest.markets.is_some());
    assert!(manifest.installers[0].signature_sha_256.is_some());
}

#[test]
fn optimize_preserves_already_optimized_manifests() {
    for fixture in [INSTALLER_MULTI, INSTALLER_ZIP_PORTABLE, INSTALLER_MSIX] {
        let original = serde_yaml::from_str::<InstallerManifest>(fixture).unwrap();

        let mut optimized = original.clone();
        optimized.optimize();

        assert_eq!(optimized, original);
    }
}

#[test]
fn schema_urls_match_manifest_version() {
    let version = winget_types::ManifestVersion::DEFAULT.to_string();

    for schema in [
        InstallerManifest::SCHEMA,
        DefaultLocaleManifest::SCHEMA,
        LocaleManifest::SCHEMA,
        VersionManifest::SCHEMA,
    ] {
        assert!(
            schema.ends_with(&format!(".{version}.schema.json")),
            "{schema} does not match manifest version {version}"
        );
    }
}

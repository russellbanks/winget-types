use super::{
    LanguageTag, Manifest, ManifestType, ManifestVersion, PackageIdentifier, PackageVersion,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct VersionManifest {
    /// The unique identifier for a given package.
    ///
    /// This value is generally in the form of `Publisher.Package`. It is case-sensitive, and this
    /// value must match the folder structure under the partition directory in GitHub.
    pub package_identifier: PackageIdentifier,

    /// The version of the package.
    ///
    /// It is related to the specific release this manifests targets. In some cases you will see a
    /// perfectly formed [semantic version] number, and in other cases you might see something
    /// different. These may be date driven, or they might have other characters with some package
    /// specific meaning for example.
    ///
    /// The Windows Package Manager client uses this version to determine if an upgrade for a
    /// package is available. In some cases, packages may be released with a marketing driven
    /// version, and that causes trouble with the [`winget upgrade`] command.
    ///
    /// The current best practice is to use the value reported in Add / Remove Programs when this
    /// version of the package is installed. In some cases, packages do not report a version
    /// resulting in an upgrade loop or other unwanted behavior.
    ///
    /// [semantic version]: https://semver.org/
    /// [`winget upgrade`]: https://docs.microsoft.com/windows/package-manager/winget/upgrade
    pub package_version: PackageVersion,

    /// The default locale for package meta-data.
    ///
    /// The format is BCP-47. This value identifies the language for meta-data to be displayed to a
    /// user when no locale file matching their preferences is available.
    ///
    /// The validation pipelines use this value to ensure the corresponding locale file is present
    /// and conforms with the defaultLocale YAML specification.
    pub default_locale: LanguageTag,

    /// The manifest type.
    ///
    /// Must have the value [`version`]. The Microsoft community package repository validation
    /// pipelines also use this value to determine appropriate validation rules when evaluating this
    /// file.
    ///
    /// [`version`]: ManifestType::Version
    #[cfg_attr(feature = "serde", serde(default = "ManifestType::version"))]
    pub manifest_type: ManifestType,

    /// The manifest syntax version.
    ///
    /// Must have the value `1.10.0`. The Microsoft community package repository validation
    /// pipelines also use this value to determine appropriate validation rules when evaluating this
    /// file.
    #[cfg_attr(feature = "serde", serde(default))]
    pub manifest_version: ManifestVersion,
}

impl VersionManifest {
    pub fn update(&mut self, package_version: &PackageVersion) {
        self.package_version.clone_from(package_version);
        self.manifest_type = ManifestType::Version;
        self.manifest_version = ManifestVersion::default();
    }
}

impl Manifest for VersionManifest {
    const SCHEMA: &'static str = "https://aka.ms/winget-manifest.version.1.10.0.schema.json";

    const TYPE: ManifestType = ManifestType::Version;
}

impl Default for VersionManifest {
    fn default() -> Self {
        Self {
            package_identifier: PackageIdentifier::default(),
            package_version: PackageVersion::default(),
            default_locale: LanguageTag::default(),
            manifest_type: ManifestType::Version,
            manifest_version: ManifestVersion::default(),
        }
    }
}

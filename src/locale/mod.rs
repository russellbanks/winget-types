mod agreement;
mod author;
mod copyright;
mod description;
mod documentation;
mod icon;
mod installation_notes;
mod license;
mod moniker;
mod package_name;
mod publisher;
mod release_notes;
mod short_description;
mod tag;

use alloc::collections::BTreeSet;

pub use agreement::Agreement;
pub use author::{Author, AuthorError};
pub use copyright::{Copyright, CopyrightError};
pub use description::{Description, DescriptionError};
pub use documentation::{DocumentLabel, Documentation};
pub use icon::Icon;
pub use installation_notes::{InstallationNotes, InstallationNotesError};
pub use license::{License, LicenseError};
pub use moniker::Moniker;
pub use package_name::{PackageName, PackageNameError};
pub use publisher::{Publisher, PublisherError};
pub use release_notes::{ReleaseNotes, ReleaseNotesError};
pub use short_description::{ShortDescription, ShortDescriptionError};
pub use tag::{Tag, TagError};
use url::Url;

use super::{
    LanguageTag, Manifest, ManifestType, ManifestVersion, PackageIdentifier, PackageVersion,
    url::{
        CopyrightUrl, LicenseUrl, PackageUrl, PublisherSupportUrl, PublisherUrl, ReleaseNotesUrl,
    },
};

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct DefaultLocaleManifest {
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

    /// The locale for package metadata.
    ///
    /// The format is BCP-47. This value identifies the language for meta-data to be displayed to a
    /// user when no locale file matching their preferences is available. The Microsoft community
    /// package repository validation pipelines also use this value to determine appropriate
    /// validation rules for this file.
    pub package_locale: LanguageTag,

    /// The name of the publisher for a given package.
    ///
    /// This field is intended to allow the full publisher's or ISV's name to be displayed as they
    /// wish.
    ///
    /// With the 1.9 release of the Windows Package Manager, this name affects how packages from a
    /// source are mapped to Apps installed in Windows 10 and Windows 11 via Add / Remove Programs
    /// (ARP) and Windows Apps & Features respectively. The best practice is to ensure this matches
    /// the entry for the package when it has been installed. This should be the value of the
    /// `Publisher` sub-key for the package in the [Windows registry]. The impact is associated with
    /// `winget upgrade` and `winget list`.
    ///
    /// [Windows registry]: https://learn.microsoft.com/windows/win32/msi/uninstall-registry-key
    pub publisher: Publisher,

    /// The website for the publisher or ISV.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub publisher_url: Option<PublisherUrl>,

    /// The website for the publisher or ISV.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub publisher_support_url: Option<PublisherSupportUrl>,

    /// The privacy website or specific web page provided the publisher or ISV.
    ///
    /// If there is a privacy website or specific web page for the package it is preferred over a
    /// generic privacy page for the publisher.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub privacy_url: Option<Url>,

    /// The author of a package.
    ///
    /// In some cases, the author is an individual who develops and or maintains the package. In
    /// other cases this may be a URL pointing to the contributors web page for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub author: Option<Author>,

    /// The name of the package.
    ///
    /// This field is intended to allow the full package name to be displayed as the publisher or
    /// ISV wishes.
    ///
    /// With the 1.9 release of the Windows Package Manager, this name affects how packages from a
    /// source are mapped to Apps installed in Windows 10 via Add / Remove Programs (ARP). The best
    /// practice is to ensure this matches the ARP entry for the package name when it has been
    /// installed. This should be the value of the `DisplayName` subkey for the package in the
    /// [Windows registry]. The impact is associated with `winget upgrade` and `winget list`.
    ///
    /// [Windows registry]: https://learn.microsoft.com/windows/win32/msi/uninstall-registry-key
    pub package_name: PackageName,

    /// The website for the package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub package_url: Option<PackageUrl>,

    /// The license governing the use and or distribution for the product.
    ///
    /// This could be an open source license, or a commercial license. Please note that a copyright
    /// is not considered a license. If there is no available information on a product's license,
    /// [`Proprietary`] should be the value in this field.
    ///
    /// [`Proprietary`]: License::PROPRIETARY
    pub license: License,

    /// The license website or specific web page provided the publisher or ISV.
    ///
    /// If there is a license website or specific web page for the package it is preferred over a
    /// generic license page for the publisher.
    ///
    /// If this is a link to the license file for an open source project, it should be specific to
    /// the version for the package. Some open source projects change their license over time.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub license_url: Option<LicenseUrl>,

    /// The copyright for the package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub copyright: Option<Copyright>,

    /// The copyright website or specific web page provided the publisher or ISV.
    ///
    /// If there is a copyright website or specific web page for the package it is preferred over a
    /// generic copyright page for the publisher.
    ///
    /// If this is a link to the copyright file for an open source project, it should be specific to
    /// the version for the package. Some open source projects change their copyright over time.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub copyright_url: Option<CopyrightUrl>,

    /// The description for a package.
    ///
    /// It is intended for use in `winget show` to help a user understand what the package is.
    ///
    /// This should be something descriptive about what the package does, and it should not simply
    /// state something like `<package name> installer` or `<package name> setup`.
    pub short_description: ShortDescription,

    /// The full or long description for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<Description>,

    /// The most common term users would search for when installing or upgrading a package.
    ///
    /// If only one package uses this moniker, then the [install], [list] and [upgrade] command may
    /// match with this package.
    ///
    /// Moniker is the third property evaluated when searching for a matching package.
    ///
    /// [install]: https://docs.microsoft.com/windows/package-manager/winget/install
    /// [list]: https://docs.microsoft.com/windows/package-manager/winget/list
    /// [upgrade]: https://docs.microsoft.com/windows/package-manager/winget/upgrade
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub moniker: Option<Moniker>,

    /// Other common term users would search for when looking for packages.
    ///
    /// Tags should be pertinent to what a user might search for when looking for a specific
    /// package.
    ///
    /// The best practice is to present these terms in all lower case with hyphens rather than
    /// spaces.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub tags: BTreeSet<Tag>,

    /// Any agreements a user must accept prior to download and subsequent install or upgrade.
    ///
    /// Agreements are only allowed in the community repository when the manifest is maintained by a
    /// verified developer.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub agreements: BTreeSet<Agreement>,

    /// The release notes for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub release_notes: Option<ReleaseNotes>,

    /// Release notes webpage for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub release_notes_url: Option<ReleaseNotesUrl>,

    /// The purchase url for acquiring entitlement for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub purchase_url: Option<Url>,

    /// The notes displayed to the user upon completion of a package installation.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub installation_notes: Option<InstallationNotes>,

    /// Any documentation for providing software guides such as manuals and troubleshooting URLs.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub documentations: BTreeSet<Documentation>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub icons: BTreeSet<Icon>,

    /// The manifest type.
    ///
    /// Must have the value [`defaultLocale`]. The Microsoft community package repository validation
    /// pipelines also use this value to determine appropriate validation rules when evaluating this
    /// file.
    ///
    /// [`defaultLocale`]: ManifestType::DefaultLocale
    #[cfg_attr(feature = "serde", serde(default = "ManifestType::default_locale"))]
    pub manifest_type: ManifestType,

    /// The manifest syntax version.
    ///
    /// Must have the value `1.12.0`. The Microsoft community package repository validation
    /// pipelines also use this value to determine appropriate validation rules when evaluating this
    /// file.
    #[cfg_attr(feature = "serde", serde(default))]
    pub manifest_version: ManifestVersion,
}

impl Manifest for DefaultLocaleManifest {
    const SCHEMA: &'static str = "https://aka.ms/winget-manifest.defaultLocale.1.12.0.schema.json";

    const TYPE: ManifestType = ManifestType::DefaultLocale;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct LocaleManifest {
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

    /// The locale for package metadata.
    ///
    /// The format is BCP-47. This value identifies the language for meta-data to be displayed to a
    /// user when no locale file matching their preferences is available. The Microsoft community
    /// package repository validation pipelines also use this value to determine appropriate
    /// validation rules for this file.
    pub package_locale: LanguageTag,

    /// The name of the publisher for a given package.
    ///
    /// This field is intended to allow the full publisher's or ISV's name to be displayed as they
    /// wish.
    ///
    /// With the 1.9 release of the Windows Package Manager, this name affects how packages from a
    /// source are mapped to Apps installed in Windows 10 and Windows 11 via Add / Remove Programs
    /// (ARP) and Windows Apps & Features respectively. The best practice is to ensure this matches
    /// the entry for the package when it has been installed. This should be the value of the
    /// `Publisher` sub-key for the package in the [Windows registry]. The impact is associated with
    /// `winget upgrade` and `winget list`.
    ///
    /// [Windows registry]: https://learn.microsoft.com/windows/win32/msi/uninstall-registry-key
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub publisher: Option<Publisher>,

    /// The website for the publisher or ISV.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub publisher_url: Option<PublisherUrl>,

    /// The website for the publisher or ISV.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub publisher_support_url: Option<PublisherSupportUrl>,

    /// The privacy website or specific web page provided the publisher or ISV.
    ///
    /// If there is a privacy website or specific web page for the package it is preferred over a
    /// generic privacy page for the publisher.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub privacy_url: Option<Url>,

    /// The author of a package.
    ///
    /// In some cases, the author is an individual who develops and or maintains the package. In
    /// other cases this may be a URL pointing to the contributors web page for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub author: Option<Author>,

    /// The name of the package.
    ///
    /// This field is intended to allow the full package name to be displayed as the publisher or
    /// ISV wishes.
    ///
    /// With the 1.9 release of the Windows Package Manager, this name affects how packages from a
    /// source are mapped to Apps installed in Windows 10 via Add / Remove Programs (ARP). The best
    /// practice is to ensure this matches the ARP entry for the package name when it has been
    /// installed. This should be the value of the `DisplayName` subkey for the package in the
    /// [Windows registry]. The impact is associated with `winget upgrade` and `winget list`.
    ///
    /// [Windows registry]: https://learn.microsoft.com/windows/win32/msi/uninstall-registry-key
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub package_name: Option<PackageName>,

    /// The website for the package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub package_url: Option<PackageUrl>,

    /// The license governing the use and or distribution for the product.
    ///
    /// This could be an open source license, or a commercial license. Please note that a copyright
    /// is not considered a license. If there is no available information on a product's license,
    /// `Proprietary` should be the value in this field.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub license: Option<License>,

    /// The license website or specific web page provided the publisher or ISV.
    ///
    /// If there is a license website or specific web page for the package it is preferred over a
    /// generic license page for the publisher.
    ///
    /// If this is a link to the license file for an open source project, it should be specific to
    /// the version for the package. Some open source projects change their license over time.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub license_url: Option<LicenseUrl>,

    /// The copyright for the package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub copyright: Option<Copyright>,

    /// The copyright website or specific web page provided the publisher or ISV.
    ///
    /// If there is a copyright website or specific web page for the package it is preferred over a
    /// generic copyright page for the publisher.
    ///
    /// If this is a link to the copyright file for an open source project, it should be specific to
    /// the version for the package. Some open source projects change their copyright over time.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub copyright_url: Option<CopyrightUrl>,

    /// The description for a package.
    ///
    /// It is intended for use in `winget show` to help a user understand what the package is.
    ///
    /// This should be something descriptive about what the package does, and it should not simply
    /// state something like `<package name> installer` or `<package name> setup`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub short_description: Option<ShortDescription>,

    /// The full or long description for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<Description>,

    /// Other common term users would search for when looking for packages.
    ///
    /// Tags should be pertinent to what a user might search for when looking for a specific
    /// package.
    ///
    /// The best practice is to present these terms in all lower case with hyphens rather than
    /// spaces.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub tags: BTreeSet<Tag>,

    /// Any agreements a user must accept prior to download and subsequent install or upgrade.
    ///
    /// Agreements are only allowed in the community repository when the manifest is maintained by a
    /// verified developer.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub agreements: BTreeSet<Agreement>,

    /// The release notes for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub release_notes: Option<ReleaseNotes>,

    /// Release notes webpage for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub release_notes_url: Option<ReleaseNotesUrl>,

    /// The purchase url for acquiring entitlement for a package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub purchase_url: Option<Url>,

    /// The notes displayed to the user upon completion of a package installation.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub installation_notes: Option<InstallationNotes>,

    /// Any documentation for providing software guides such as manuals and troubleshooting URLs.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub documentations: BTreeSet<Documentation>,

    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub icons: BTreeSet<Icon>,

    /// The manifest type.
    ///
    /// Must have the value [`locale`]. The Microsoft community package repository validation
    /// pipelines also use this value to determine appropriate validation rules when evaluating this
    /// file.
    ///
    /// [`locale`]: ManifestType::Locale
    #[cfg_attr(feature = "serde", serde(default = "ManifestType::locale"))]
    pub manifest_type: ManifestType,

    /// The manifest syntax version.
    ///
    /// Must have the value `1.12.0`. The Microsoft community package repository validation
    /// pipelines also use this value to determine appropriate validation rules when evaluating this
    /// file.
    #[cfg_attr(feature = "serde", serde(default))]
    pub manifest_version: ManifestVersion,
}

impl Manifest for LocaleManifest {
    const SCHEMA: &'static str = "https://aka.ms/winget-manifest.locale.1.12.0.schema.json";

    const TYPE: ManifestType = ManifestType::Locale;
}

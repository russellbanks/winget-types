#![expect(clippy::struct_excessive_bools)]

mod apps_and_features_entry;
mod architecture;
pub mod authentication;
mod capability;
mod channel;
mod command;
mod dependencies;
mod elevation_requirement;
mod expected_return_codes;
mod file_extension;
mod install_modes;
mod installation_metadata;
mod installer_return_code;
mod installer_type;
mod market;
mod minimum_os_version;
mod nested;
mod platform;
mod protocol;
mod repair_behavior;
mod return_response;
mod scope;
pub mod switches;
mod unsupported_arguments;
mod unsupported_os_architectures;
mod upgrade_behavior;

use alloc::{collections::BTreeSet, string::String, vec::Vec};

pub use apps_and_features_entry::AppsAndFeaturesEntry;
pub use architecture::{Architecture, ParseArchitectureError};
pub use authentication::Authentication;
pub use capability::{Capability, CapabilityError, RestrictedCapability};
pub use channel::{Channel, ChannelError};
use chrono::NaiveDate;
pub use command::{Command, CommandError};
pub use dependencies::{Dependencies, PackageDependencies};
pub use elevation_requirement::ElevationRequirement;
pub use expected_return_codes::ExpectedReturnCodes;
pub use file_extension::{FileExtension, FileExtensionError};
pub use install_modes::InstallModes;
pub use installation_metadata::InstallationMetadata;
pub use installer_return_code::{InstallerReturnCode, InstallerSuccessCode};
pub use installer_type::InstallerType;
use itertools::Itertools;
pub use market::{Market, MarketError, Markets, MarketsError};
pub use minimum_os_version::{MinimumOSVersion, MinimumOSVersionError};
use nested::installer_type::NestedInstallerType;
pub use nested::{
    PortableCommandAlias, PortableCommandAliasError, installer_files::NestedInstallerFiles,
};
pub use package_family_name::PackageFamilyName;
pub use platform::{Platform, PlatformParseError};
pub use protocol::{Protocol, ProtocolError};
pub use repair_behavior::RepairBehavior;
pub use scope::{Scope, ScopeParseError};
pub use switches::InstallerSwitches;
pub use unsupported_arguments::UnsupportedArguments;
pub use unsupported_os_architectures::UnsupportedOSArchitecture;
pub use upgrade_behavior::{UpgradeBehavior, UpgradeBehaviorParseError};

use super::{
    LanguageTag, Manifest, ManifestType, ManifestVersion, PackageIdentifier, PackageVersion,
    Sha256String, url::DecodedUrl,
};

pub const VALID_FILE_EXTENSIONS: [&str; 7] = [
    "msix",
    "msi",
    "appx",
    "exe",
    "zip",
    "msixbundle",
    "appxbundle",
];

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct InstallerManifest {
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

    /// The distribution channel for a package.
    ///
    /// Examples may include "stable" or "beta".
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub channel: Option<Channel>,

    /// The locale for an installer not the package meta-data.
    ///
    /// Some installers are compiled with locale or language specific properties. If this key is
    /// present, it is used to represent the package locale for an installer.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "InstallerLocale", skip_serializing_if = "Option::is_none")
    )]
    pub locale: Option<LanguageTag>,

    /// The Windows platform targeted by the installer.
    ///
    /// The Windows Package Manager currently supports "Windows.Desktop" and "Windows.Universal".
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Platform::is_empty", default)
    )]
    pub platform: Platform,

    /// The minimum version of the Windows operating system supported by the package.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "MinimumOSVersion", skip_serializing_if = "Option::is_none")
    )]
    pub minimum_os_version: Option<MinimumOSVersion>,

    /// The installer type for the package.
    ///
    /// The Windows Package Manager supports [MSIX], [MSI], and executable installers. Some well
    /// known formats ([Inno], [Nullsoft], [WiX], and [Burn]) provide standard sets of installer
    /// switches to provide different installer experiences. Portable packages are supported as of
    /// Windows Package Manager 1.3. Zip packages are supported as of Windows Package Manager 1.5.
    ///
    /// [MSIX]: https://docs.microsoft.com/windows/msix/overview
    /// [MSI]: https://docs.microsoft.com/windows/win32/msi/windows-installer-portal
    /// [Inno]: https://jrsoftware.org/isinfo.php
    /// [Nullsoft]: https://sourceforge.net/projects/nsis
    /// [WiX]: https://wixtoolset.org/
    /// [Burn]: https://wixtoolset.org/docs/v3/bundle/
    #[cfg_attr(
        feature = "serde",
        serde(rename = "InstallerType", skip_serializing_if = "Option::is_none")
    )]
    pub r#type: Option<InstallerType>,

    /// The installer type of the file within the archive which will be used as the installer.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub nested_installer_type: Option<NestedInstallerType>,

    /// A list of all the installers to be executed within an archive.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub nested_installer_files: BTreeSet<NestedInstallerFiles>,

    /// The scope the package is installed under.
    ///
    /// The two configurations are [`user`] and [`machine`]. Some installers support only one of
    /// these scopes while others support both via arguments passed to the installer using
    /// [`InstallerSwitches`].
    ///
    /// [`user`]: Scope::User
    /// [`machine`]: Scope::Machine
    /// [`InstallerSwitches`]: InstallerSwitches
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub scope: Option<Scope>,

    /// The install modes supported by the installer.
    ///
    /// The Microsoft community package repository requires a package support "silent" and
    /// "silent with progress". The Windows Package Manager also supports "interactive" installers.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "InstallModes::is_empty", default)
    )]
    pub install_modes: InstallModes,

    /// The set of switches passed to installers.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "InstallerSwitches",
            skip_serializing_if = "InstallerSwitches::is_empty",
            default
        )
    )]
    pub switches: InstallerSwitches,

    /// Any status codes returned by the installer representing a success condition other than zero.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "InstallerSuccessCodes",
            skip_serializing_if = "BTreeSet::is_empty",
            default
        )
    )]
    pub success_codes: BTreeSet<InstallerSuccessCode>,

    /// Any status codes returned by the installer representing a condition other than zero.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub expected_return_codes: BTreeSet<ExpectedReturnCodes>,

    /// What the Windows Package Manager should do regarding the currently installed package during
    /// a package upgrade.
    ///
    /// If the package should be uninstalled first, the [`uninstallPrevious`] value should be
    /// specified. If the package should not be upgraded through `WinGet`, the [`deny`] value should
    /// be specified.
    ///
    /// [`uninstallPrevious`]: UpgradeBehavior::UninstallPrevious
    /// [`deny`]: UpgradeBehavior::Deny
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub upgrade_behavior: Option<UpgradeBehavior>,

    /// Any commands or aliases used to execute the package after it has been installed.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub commands: BTreeSet<Command>,

    /// Any protocols (i.e. URI schemes) supported by the package. For example: `["ftp", "ldap"]`.
    /// Entries shouldn't have trailing colons. The Windows Package Manager does not support any
    /// behavior related to protocols handled by a package.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub protocols: BTreeSet<Protocol>,

    /// Any file extensions supported by the package.
    ///
    /// For example: `["html", "jpg"]`. Entries shouldn't have leading dots. The Windows Package
    /// Manager does not support any behavior related to the file extensions supported by the
    /// package.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub file_extensions: BTreeSet<FileExtension>,

    /// Any dependencies required to install or run the package.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Dependencies::is_empty", default)
    )]
    pub dependencies: Dependencies,

    /// The [package family name] specified in an MSIX installer.
    ///
    /// This value is used to assist with matching packages from a source to the program installed
    /// in Windows via Add / Remove Programs for list, and upgrade behavior.
    ///
    /// [package family name]: https://learn.microsoft.com/windows/apps/desktop/modernize/package-identity-overview#package-family-name
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub package_family_name: Option<PackageFamilyName>,

    /// The [product code].
    ///
    /// This value is used to assist with matching packages from a source to the program installed
    /// in Windows via Add / Remove Programs for list, and upgrade behavior.
    ///
    /// [product code]: https://learn.microsoft.com/windows/win32/msi/product-codes
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub product_code: Option<String>,

    /// The capabilities provided by an MSIX package.
    ///
    /// More information is available for [App capability declarations].
    ///
    /// [App capability declarations]: https://docs.microsoft.com/windows/uwp/packaging/app-capability-declarations
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub capabilities: BTreeSet<Capability>,

    /// The restricted capabilities provided by an MSIX package.
    ///
    /// More information is available for [App capability declarations].
    ///
    /// [App capability declarations]: https://docs.microsoft.com/windows/uwp/packaging/app-capability-declarations
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub restricted_capabilities: BTreeSet<RestrictedCapability>,

    /// Any markets a package may or may not be installed in.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub markets: Option<Markets>,

    /// The behavior associated with installers that abort the terminal.
    ///
    /// This most often occurs when a user is performing an upgrade of the running terminal.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "InstallerAbortsTerminal",
            skip_serializing_if = "core::ops::Not::not",
            default
        )
    )]
    pub aborts_terminal: bool,

    /// The release date for a package, in RFC 3339 / ISO 8601 format, i.e. "YYYY-MM-DD".
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub release_date: Option<NaiveDate>,

    /// The requirement to have an install location specified.
    ///
    /// These installers are known to deploy files to the location the installer is executed in.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub install_location_required: bool,

    /// Identifies packages that upgrade themselves.
    ///
    /// By default, they are excluded from `winget upgrade --all`.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub require_explicit_upgrade: bool,

    /// Whether a warning message is displayed to the user prior to install or upgrade if the
    /// package is known to interfere with any running applications.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub display_install_warnings: bool,

    /// Any architectures a package is known not to be compatible with.
    ///
    /// Generally, this is associated with emulation modes.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "UnsupportedOSArchitectures",
            skip_serializing_if = "UnsupportedOSArchitecture::is_empty",
            default
        )
    )]
    pub unsupported_os_architectures: UnsupportedOSArchitecture,

    /// The list of Windows Package Manager Client arguments the installer does not support.
    ///
    /// Only the `--log` and `--location` arguments can be specified as unsupported arguments for an
    /// installer.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "UnsupportedArguments::is_empty", default)
    )]
    pub unsupported_arguments: UnsupportedArguments,

    /// The values reported by Windows Apps & Features.
    ///
    /// When a package is installed, entries are made into the Windows Registry.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub apps_and_features_entries: Vec<AppsAndFeaturesEntry>,

    /// The scope in which scope a package is required to be executed under.
    ///
    /// Some packages require user level execution while others require administrative level
    /// execution.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub elevation_requirement: Option<ElevationRequirement>,

    /// Allows for additional metadata to be used for deeper installation detection.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "InstallationMetadata::is_empty", default)
    )]
    pub installation_metadata: InstallationMetadata,

    /// When true, this flag will prohibit the manifest from being downloaded for offline
    /// installation with the winget download command.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub download_command_prohibited: bool,

    /// This field controls what method is used to repair existing installations of packages.
    ///
    /// Specifying `modify` will use the `ModifyPath` string from the package's ARP data,
    /// `uninstaller` will use the Uninstall string from the package's ARP data, and `installer`
    /// will download and run the installer. In each case, the `Repair` value from
    /// `InstallerSwitches` will be added as an argument when invoking the command to repair the
    /// package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub repair_behavior: Option<RepairBehavior>,

    /// This field controls the behavior of environment variables when installing portable packages
    /// from an archive (i.e. `zip`).
    ///
    /// Specifying `true` will add the install location directly to the `PATH` environment variable.
    /// Specifying `false` will use the default behavior of adding a symlink to the `links` folder,
    /// if supported, or adding the install location directly to `PATH` if symlinks are not
    /// supported.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub archive_binaries_depend_on_path: bool,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub authentication: Option<Authentication>,

    pub installers: Vec<Installer>,

    /// The manifest type.
    ///
    /// Must have the value [`installer`]. The Microsoft community package repository validation
    /// pipelines also use this value to determine appropriate validation rules when evaluating this
    /// file.
    ///
    /// [`installer`]: ManifestType::Installer
    #[cfg_attr(feature = "serde", serde(default = "ManifestType::installer"))]
    pub manifest_type: ManifestType,

    /// The manifest syntax version.
    ///
    /// Must have the value `1.10.0`. The Microsoft community package repository validation
    /// pipelines also use this value to determine appropriate validation rules when evaluating this
    /// file.
    #[cfg_attr(feature = "serde", serde(default))]
    pub manifest_version: ManifestVersion,
}

impl Manifest for InstallerManifest {
    const SCHEMA: &'static str = "https://aka.ms/winget-manifest.installer.1.10.0.schema.json";
    const TYPE: ManifestType = ManifestType::Installer;
}

impl InstallerManifest {
    #[expect(
        clippy::cognitive_complexity,
        reason = "The resulting complexity is generated by a macro"
    )]
    pub fn optimize(&mut self) {
        macro_rules! optimize_keys {
            ($($($field:ident).+),* $(,)?) => {
                #[inline]
                fn default<T: Default>(_: &T) -> T {
                    T::default()
                }

                $(
                    if let Ok(field) = self
                        .installers
                        .iter_mut()
                        .map(|installer| &mut installer.$($field).+)
                        .all_equal_value()
                    {
                        self.$($field).+ = core::mem::take(r#field);
                        for installer in &mut self.installers {
                            installer.$($field).+ = default(&installer.$($field).+);
                        }
                    } else {
                        self.$($field).+ = default(&self.$($field).+);
                    }
                )*
            };
        }

        optimize_keys!(
            locale,
            platform,
            minimum_os_version,
            r#type,
            nested_installer_type,
            nested_installer_files,
            scope,
            install_modes,
            switches.silent,
            switches.silent_with_progress,
            switches.interactive,
            switches.install_location,
            switches.log,
            switches.upgrade,
            switches.repair,
            success_codes,
            expected_return_codes,
            upgrade_behavior,
            commands,
            protocols,
            file_extensions,
            dependencies.windows_features,
            dependencies.windows_libraries,
            dependencies.package,
            dependencies.external,
            package_family_name,
            product_code,
            capabilities,
            restricted_capabilities,
            markets,
            aborts_terminal,
            release_date,
            install_location_required,
            require_explicit_upgrade,
            display_install_warnings,
            unsupported_os_architectures,
            unsupported_arguments,
            apps_and_features_entries,
            elevation_requirement,
            installation_metadata,
            download_command_prohibited,
            repair_behavior,
            archive_binaries_depend_on_path,
        );

        self.manifest_version = ManifestVersion::default();

        self.installers.sort_unstable();
        self.installers.dedup();
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Installer {
    /// The locale for an installer *not* the package meta-data.
    ///
    /// Some installers are compiled with locale or language specific properties. If this key is
    /// present, it is used to represent the package locale for an installer.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "InstallerLocale", skip_serializing_if = "Option::is_none")
    )]
    pub locale: Option<LanguageTag>,

    /// The Windows platform targeted by the installer.
    ///
    /// The Windows Package Manager currently supports "Windows.Desktop" and "Windows.Universal".
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Platform::is_empty", default)
    )]
    pub platform: Platform,

    /// The minimum version of the Windows operating system supported by the package.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "MinimumOSVersion", skip_serializing_if = "Option::is_none")
    )]
    pub minimum_os_version: Option<MinimumOSVersion>,

    /// The hardware architecture targeted by the installer.
    ///
    /// The Windows Package Manager will attempt to determine the best architecture to use. If
    /// emulation is available and the native hardware architecture does not have a supported
    /// installer, the emulated architecture may be used.
    pub architecture: Architecture,

    /// The installer type for the package.
    ///
    /// The Windows Package Manager supports [MSIX], [MSI], and executable installers. Some well
    /// known formats ([Inno], [Nullsoft], [WiX], and [Burn]) provide standard sets of installer
    /// switches to provide different installer experiences. Portable packages are supported as of
    /// Windows Package Manager 1.3. Zip packages are supported as of Windows Package Manager 1.5.
    ///
    /// [MSIX]: https://docs.microsoft.com/windows/msix/overview
    /// [MSI]: https://docs.microsoft.com/windows/win32/msi/windows-installer-portal
    /// [Inno]: https://jrsoftware.org/isinfo.php
    /// [Nullsoft]: https://sourceforge.net/projects/nsis
    /// [WiX]: https://wixtoolset.org/
    /// [Burn]: https://wixtoolset.org/docs/v3/bundle/
    #[cfg_attr(
        feature = "serde",
        serde(rename = "InstallerType", skip_serializing_if = "Option::is_none")
    )]
    pub r#type: Option<InstallerType>,

    /// The installer type of the file within the archive which will be used as the installer.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub nested_installer_type: Option<NestedInstallerType>,

    /// A list of all the installers to be executed within an archive.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub nested_installer_files: BTreeSet<NestedInstallerFiles>,

    /// The scope the package is installed under.
    ///
    /// The two configurations are [`user`] and [`machine`]. Some installers support only one of
    /// these scopes while others support both via arguments passed to the installer using
    /// [`InstallerSwitches`].
    ///
    /// [`user`]: Scope::User
    /// [`machine`]: Scope::Machine
    /// [`InstallerSwitches`]: InstallerSwitches
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub scope: Option<Scope>,

    /// The URL to download the installer.
    #[cfg_attr(feature = "serde", serde(rename = "InstallerUrl"))]
    pub url: DecodedUrl,

    /// The SHA 256 hash for the installer. It is used to confirm the installer has not been
    /// modified. The Windows Package Manager will compare the hash in the manifest with the
    /// calculated hash of the installer after it has been downloaded.
    #[cfg_attr(feature = "serde", serde(rename = "InstallerSha256"))]
    pub sha_256: Sha256String,

    /// The signature file (AppxSignature.p7x) inside an MSIX installer. It is used to provide
    /// streaming install for MSIX packages.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub signature_sha_256: Option<Sha256String>,

    /// The install modes supported by the installer.
    ///
    /// The Microsoft community package repository requires a package support "silent" and
    /// "silent with progress". The Windows Package Manager also supports "interactive" installers.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "InstallModes::is_empty", default)
    )]
    pub install_modes: InstallModes,

    /// The set of switches passed to installers.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "InstallerSwitches",
            skip_serializing_if = "InstallerSwitches::is_empty",
            default
        )
    )]
    pub switches: InstallerSwitches,

    /// Any status codes returned by the installer representing a success condition other than zero.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "InstallerSuccessCodes",
            skip_serializing_if = "BTreeSet::is_empty",
            default
        )
    )]
    pub success_codes: BTreeSet<InstallerSuccessCode>,

    /// Any status codes returned by the installer representing a condition other than zero.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub expected_return_codes: BTreeSet<ExpectedReturnCodes>,

    /// What the Windows Package Manager should do regarding the currently installed package during
    /// a package upgrade.
    ///
    /// If the package should be uninstalled first, the [`uninstallPrevious`] value should be
    /// specified. If the package should not be upgraded through `WinGet`, the [`deny`] value should
    /// be specified.
    ///
    /// [`uninstallPrevious`]: UpgradeBehavior::UninstallPrevious
    /// [`deny`]: UpgradeBehavior::Deny
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub upgrade_behavior: Option<UpgradeBehavior>,

    /// Any commands or aliases used to execute the package after it has been installed.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub commands: BTreeSet<Command>,

    /// Any protocols (i.e. URI schemes) supported by the package. For example: `["ftp", "ldap"]`.
    /// Entries shouldn't have trailing colons. The Windows Package Manager does not support any
    /// behavior related to protocols handled by a package.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub protocols: BTreeSet<Protocol>,

    /// Any file extensions supported by the package.
    ///
    /// For example: `["html", "jpg"]`. Entries shouldn't have leading dots. The Windows Package
    /// Manager does not support any behavior related to the file extensions supported by the
    /// package.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub file_extensions: BTreeSet<FileExtension>,

    /// Any dependencies required to install or run the package.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Dependencies::is_empty", default)
    )]
    pub dependencies: Dependencies,

    /// The [package family name] specified in an MSIX installer.
    ///
    /// This value is used to assist with matching packages from a source to the program installed
    /// in Windows via Add / Remove Programs for list, and upgrade behavior.
    ///
    /// [package family name]: https://learn.microsoft.com/windows/apps/desktop/modernize/package-identity-overview#package-family-name
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub package_family_name: Option<PackageFamilyName>,

    /// The [product code].
    ///
    /// This value is used to assist with matching packages from a source to the program installed
    /// in Windows via Add / Remove Programs for list, and upgrade behavior.
    ///
    /// [product code]: https://learn.microsoft.com/windows/win32/msi/product-codes
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub product_code: Option<String>,

    /// The capabilities provided by an MSIX package.
    ///
    /// More information is available for [App capability declarations].
    ///
    /// [App capability declarations]: https://docs.microsoft.com/windows/uwp/packaging/app-capability-declarations
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub capabilities: BTreeSet<Capability>,

    /// The restricted capabilities provided by an MSIX package.
    ///
    /// More information is available for [App capability declarations].
    ///
    /// [App capability declarations]: https://docs.microsoft.com/windows/uwp/packaging/app-capability-declarations
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub restricted_capabilities: BTreeSet<RestrictedCapability>,

    /// Any markets a package may or may not be installed in.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub markets: Option<Markets>,

    /// The behavior associated with installers that abort the terminal.
    ///
    /// This most often occurs when a user is performing an upgrade of the running terminal.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "InstallerAbortsTerminal",
            skip_serializing_if = "core::ops::Not::not",
            default
        )
    )]
    pub aborts_terminal: bool,

    /// The release date for a package, in RFC 3339 / ISO 8601 format, i.e. "YYYY-MM-DD".
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub release_date: Option<NaiveDate>,

    /// The requirement to have an install location specified.
    ///
    /// These installers are known to deploy files to the location the installer is executed in.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub install_location_required: bool,

    /// Identifies packages that upgrade themselves.
    ///
    /// By default, they are excluded from `winget upgrade --all`.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub require_explicit_upgrade: bool,

    /// Whether a warning message is displayed to the user prior to install or upgrade if the
    /// package is known to interfere with any running applications.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub display_install_warnings: bool,

    /// Any architectures a package is known not to be compatible with.
    ///
    /// Generally, this is associated with emulation modes.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "UnsupportedOSArchitectures",
            skip_serializing_if = "UnsupportedOSArchitecture::is_empty",
            default
        )
    )]
    pub unsupported_os_architectures: UnsupportedOSArchitecture,

    /// The list of Windows Package Manager Client arguments the installer does not support.
    ///
    /// Only the `--log` and `--location` arguments can be specified as unsupported arguments for an
    /// installer.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "UnsupportedArguments::is_empty", default)
    )]
    pub unsupported_arguments: UnsupportedArguments,

    /// The values reported by Windows Apps & Features.
    ///
    /// When a package is installed, entries are made into the Windows Registry.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub apps_and_features_entries: Vec<AppsAndFeaturesEntry>,

    /// The scope in which scope a package is required to be executed under.
    ///
    /// Some packages require user level execution while others require administrative level
    /// execution.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub elevation_requirement: Option<ElevationRequirement>,

    /// Allows for additional metadata to be used for deeper installation detection.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "InstallationMetadata::is_empty", default)
    )]
    pub installation_metadata: InstallationMetadata,

    /// When true, this flag will prohibit the manifest from being downloaded for offline
    /// installation with the winget download command.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub download_command_prohibited: bool,

    /// This field controls what method is used to repair existing installations of packages.
    ///
    /// Specifying `modify` will use the `ModifyPath` string from the package's ARP data,
    /// `uninstaller` will use the Uninstall string from the package's ARP data, and `installer`
    /// will download and run the installer. In each case, the `Repair` value from
    /// `InstallerSwitches` will be added as an argument when invoking the command to repair the
    /// package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub repair_behavior: Option<RepairBehavior>,

    /// This field controls the behavior of environment variables when installing portable packages
    /// from an archive (i.e. `zip`).
    ///
    /// Specifying `true` will add the install location directly to the `PATH` environment variable.
    /// Specifying `false` will use the default behavior of adding a symlink to the `links` folder,
    /// if supported, or adding the install location directly to `PATH` if symlinks are not
    /// supported.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "core::ops::Not::not", default)
    )]
    pub archive_binaries_depend_on_path: bool,

    /// This field controls the authentication for Entra ID secured private sources.
    ///
    /// Resource and scope information can be included if a specific resource is needed to download
    /// or install the package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub authentication: Option<Authentication>,
}

impl Installer {
    /// Merges two installers.
    ///
    /// If a key of `self` is equal to its default, it will take the value from `other`. If the key
    /// of `self` is not equal to its default, it will retain that value and the equivalent key in
    /// `other` is ignored.
    #[expect(
        clippy::cognitive_complexity,
        reason = "The resulting complexity is generated by a macro"
    )]
    #[must_use]
    pub fn merge_with(mut self, other: Self) -> Self {
        macro_rules! merge_keys {
            (
                $($($field:ident).+),*,
                [$($switch:ident),* $(,)?]$(,)?
            ) => {
                #[inline]
                fn default<T: Default>(_: &T) -> T {
                    T::default()
                }

                $(
                    if self.$($field).+ == default(&self.$($field).+) {
                        self.$($field).+ = other.$($field).+;
                    }
                )*

                $(
                    if let (Some(switch), Some(other_switch)) = (
                        self.switches.$switch.as_mut(),
                        other.switches.$switch.as_ref(),
                    ) {
                        for part in other_switch {
                            if !switch.contains(part) {
                                switch.push(part.clone());
                            }
                        }
                    }
                )*
            };
        }

        merge_keys!(
            locale,
            platform,
            minimum_os_version,
            r#type,
            nested_installer_type,
            nested_installer_files,
            scope,
            install_modes,
            success_codes,
            expected_return_codes,
            upgrade_behavior,
            commands,
            protocols,
            file_extensions,
            dependencies,
            package_family_name,
            product_code,
            capabilities,
            restricted_capabilities,
            markets,
            aborts_terminal,
            release_date,
            install_location_required,
            require_explicit_upgrade,
            display_install_warnings,
            unsupported_os_architectures,
            unsupported_arguments,
            apps_and_features_entries,
            elevation_requirement,
            installation_metadata,
            download_command_prohibited,
            repair_behavior,
            archive_binaries_depend_on_path,
            [
                silent,
                silent_with_progress,
                interactive,
                install_location,
                log,
                upgrade,
                custom,
                repair
            ],
        );

        self
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::{
        installer::{Architecture, Installer, InstallerManifest, InstallerSwitches},
        shared::LanguageTag,
    };

    #[test]
    fn optimize_duplicate_locale() {
        let mut manifest = InstallerManifest {
            installers: vec![
                Installer {
                    locale: Some("en-US".parse::<LanguageTag>().unwrap()),
                    architecture: Architecture::X86,
                    ..Installer::default()
                },
                Installer {
                    locale: Some("en-US".parse::<LanguageTag>().unwrap()),
                    architecture: Architecture::X64,
                    ..Installer::default()
                },
            ],
            ..InstallerManifest::default()
        };

        manifest.optimize();

        assert_eq!(
            manifest,
            InstallerManifest {
                locale: Some("en-US".parse::<LanguageTag>().unwrap()),
                installers: vec![
                    Installer {
                        architecture: Architecture::X86,
                        ..Installer::default()
                    },
                    Installer {
                        architecture: Architecture::X64,
                        ..Installer::default()
                    },
                ],
                ..InstallerManifest::default()
            }
        )
    }

    #[test]
    fn optimize_duplicate_switch() {
        let mut manifest = InstallerManifest {
            installers: vec![
                Installer {
                    architecture: Architecture::X86,
                    switches: InstallerSwitches::new()
                        .silent("--silent")
                        .custom("--custom"),
                    ..Installer::default()
                },
                Installer {
                    architecture: Architecture::X64,
                    switches: InstallerSwitches::new().silent("--silent"),
                    ..Installer::default()
                },
            ],
            ..InstallerManifest::default()
        };

        manifest.optimize();

        assert_eq!(
            manifest,
            InstallerManifest {
                switches: InstallerSwitches::new().silent("--silent"),
                installers: vec![
                    Installer {
                        architecture: Architecture::X86,
                        switches: InstallerSwitches::new().custom("--custom"),
                        ..Installer::default()
                    },
                    Installer {
                        architecture: Architecture::X64,
                        ..Installer::default()
                    },
                ],
                ..InstallerManifest::default()
            }
        )
    }
}

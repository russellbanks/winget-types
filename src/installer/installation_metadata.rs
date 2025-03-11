use alloc::{collections::BTreeSet, string::String};
use core::fmt;

use camino::Utf8PathBuf;

use super::Sha256String;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct InstallationMetadata {
    /// The default install location for the package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub default_install_location: Option<Utf8PathBuf>,

    /// The files installed for the package.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub files: BTreeSet<MetadataFiles>,
}

impl InstallationMetadata {
    /// Returns `true` if `default_install_location` is `None` and `files` is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.default_install_location.is_none() && self.files.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct MetadataFiles {
    /// The path to the installed file relative to the default install location.
    pub relative_file_path: Utf8PathBuf,

    /// The Sha256 hash of the installed file.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub file_sha_256: Option<Sha256String>,

    /// The type of the installed file - [`launch`], [`uninstall`], or [`other`]. If not specified,
    /// the file is treated as [`other`].
    ///
    /// [`launch`]: MetadataFileType::Launch
    /// [`uninstall`]: MetadataFileType::Uninstall
    /// [`other`]: MetadataFileType::Other
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub file_type: Option<MetadataFileType>,

    /// The parameter to use for invocable files.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub invocation_parameter: Option<String>,

    /// The display name to use for invocable files.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub display_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum MetadataFileType {
    Launch,
    Uninstall,
    Other,
}

impl fmt::Display for MetadataFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Launch => write!(f, "launch"),
            Self::Uninstall => write!(f, "uninstall"),
            Self::Other => write!(f, "other"),
        }
    }
}

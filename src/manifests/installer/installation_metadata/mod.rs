mod installed_file;

use alloc::collections::BTreeSet;

use installed_file::InstalledFile;

use crate::PathBuf;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct InstallationMetadata {
    /// The default install location for the package.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    default_install_location: Option<PathBuf>,

    /// The files installed for the package.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    files: BTreeSet<InstalledFile>,
}

impl InstallationMetadata {
    /// Creates a new [`InstallationMetadata`] from a default install location
    /// and an iterator of [`InstalledFile`].
    #[must_use]
    pub fn new<P, I>(default_install_location: P, files: I) -> Self
    where
        P: Into<Option<PathBuf>>,
        I: IntoIterator<Item = InstalledFile>,
    {
        Self {
            default_install_location: default_install_location.into(),
            files: files.into_iter().collect(),
        }
    }

    /// Creates a new [`InstallationMetadata`] from a default install location.
    #[must_use]
    pub fn new_install_location<P>(default_install_location: P) -> Self
    where
        P: Into<Option<PathBuf>>,
    {
        Self {
            default_install_location: default_install_location.into(),
            files: BTreeSet::new(),
        }
    }

    /// Returns `true` if `default_install_location` is `None` and `files` is
    /// empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.default_install_location.is_none() && self.files.is_empty()
    }
}

use alloc::{collections::BTreeSet, string::String};

pub use package::PackageDependencies;

mod package;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Dependencies {
    /// List of Windows feature dependencies
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub windows_features: BTreeSet<String>,

    /// List of Windows library dependencies
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    pub windows_libraries: BTreeSet<String>,

    /// List of package dependencies from current source
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "PackageDependencies",
            skip_serializing_if = "BTreeSet::is_empty",
            default
        )
    )]
    pub package: BTreeSet<PackageDependencies>,

    /// List of external package dependencies
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "ExternalDependencies",
            skip_serializing_if = "BTreeSet::is_empty",
            default
        )
    )]
    pub external: BTreeSet<String>,
}

impl Dependencies {
    /// Returns `true` if all the dependency fields are empty.
    ///
    /// # Examples
    /// ```
    /// # use std::collections::BTreeSet;
    /// # use winget_types::installer::{Dependencies, PackageDependencies};
    /// # use winget_types::{PackageIdentifier, PackageIdentifierError};
    ///
    /// # fn main() -> Result<(), PackageIdentifierError> {
    /// let mut dependencies = Dependencies {
    ///     windows_features: BTreeSet::new(),
    ///     windows_libraries: BTreeSet::new(),
    ///     package: BTreeSet::new(),
    ///     external: BTreeSet::new(),
    /// };
    ///
    /// assert!(dependencies.is_empty());
    ///
    /// dependencies.package.insert(PackageDependencies::new(PackageIdentifier::new("Git.Git")?));
    ///
    /// assert!(!dependencies.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.windows_features.is_empty()
            && self.windows_libraries.is_empty()
            && self.package.is_empty()
            && self.external.is_empty()
    }
}

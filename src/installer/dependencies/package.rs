use crate::shared::{PackageIdentifier, PackageVersion};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct PackageDependencies {
    pub package_identifier: PackageIdentifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub minimum_version: Option<PackageVersion>,
}

impl PackageDependencies {
    /// Creates a new `PackageDependencies` from a [`PackageIdentifier`].
    #[must_use]
    pub const fn new(package_identifier: PackageIdentifier) -> Self {
        Self {
            package_identifier,
            minimum_version: None,
        }
    }

    /// Creates a new `PackageDependencies` from a [`PackageIdentifier`] and a minimum version.
    #[must_use]
    pub const fn new_with_min_version(
        package_identifier: PackageIdentifier,
        minimum_version: PackageVersion,
    ) -> Self {
        Self {
            package_identifier,
            minimum_version: Some(minimum_version),
        }
    }
}

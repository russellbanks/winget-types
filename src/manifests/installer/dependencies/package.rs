use crate::{PackageIdentifier, PackageVersion};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct PackageDependency {
    package_identifier: PackageIdentifier,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    minimum_version: Option<PackageVersion>,
}

impl PackageDependency {
    /// Creates a new [`PackageDependency`] from a [`PackageIdentifier`].
    #[must_use]
    #[inline]
    pub const fn new(package_identifier: PackageIdentifier) -> Self {
        Self {
            package_identifier,
            minimum_version: None,
        }
    }

    /// Creates a new `PackageDependency` from a [`PackageIdentifier`] and a
    /// minimum version.
    #[must_use]
    #[inline]
    pub const fn new_with_min_version(
        package_identifier: PackageIdentifier,
        minimum_version: PackageVersion,
    ) -> Self {
        Self {
            package_identifier,
            minimum_version: Some(minimum_version),
        }
    }

    /// Returns the package dependency's package identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::PackageDependency;
    /// # use winget_types::{PackageIdentifier, PackageIdentifierError};
    ///
    /// # fn main() -> Result<(), PackageIdentifierError> {
    /// let vc_redist_2015 = PackageIdentifier::new("Microsoft.VCRedist.2015+.x64")?;
    /// let package_dependency = PackageDependency::new(vc_redist_2015.clone());
    ///
    /// assert_eq!(package_dependency.package_identifier(), &vc_redist_2015);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn package_identifier(&self) -> &PackageIdentifier {
        &self.package_identifier
    }

    /// Returns the package dependency's minimum version.
    #[must_use]
    #[inline]
    pub const fn minimum_version(&self) -> Option<&PackageVersion> {
        self.minimum_version.as_ref()
    }
}

impl From<PackageIdentifier> for PackageDependency {
    #[inline]
    fn from(package_identifier: PackageIdentifier) -> Self {
        Self::new(package_identifier)
    }
}

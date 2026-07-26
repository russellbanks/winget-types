use alloc::{collections::BTreeSet, string::String};

use bon::Builder;
pub use package::PackageDependency;

mod package;

/// Any dependencies required to install or run a package.
///
/// # Examples
///
/// ```
/// use winget_types::installer::Dependencies;
/// # use winget_types::{PackageIdentifier, PackageIdentifierError};
///
/// # fn main() -> Result<(), PackageIdentifierError> {
/// let dependencies = Dependencies::builder()
///     .packages([PackageIdentifier::new("Microsoft.VCRedist.2015+.x64")
///         .unwrap()
///         .into()])
///    .build();
/// # Ok(())
/// # }
/// ```
#[derive(Builder, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[builder(on(BTreeSet<_>, into))]
pub struct Dependencies {
    /// List of Windows feature dependencies.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    #[builder(default)]
    pub windows_features: BTreeSet<String>,

    /// List of Windows library dependencies.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "BTreeSet::is_empty", default)
    )]
    #[builder(default)]
    pub windows_libraries: BTreeSet<String>,

    /// List of package dependencies from current source.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "PackageDependencies",
            skip_serializing_if = "BTreeSet::is_empty",
            default
        )
    )]
    #[builder(default)]
    pub packages: BTreeSet<PackageDependency>,

    /// List of external package dependencies.
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "ExternalDependencies",
            skip_serializing_if = "BTreeSet::is_empty",
            default
        )
    )]
    #[builder(default)]
    pub external: BTreeSet<String>,
}

impl Dependencies {
    /// Returns the windows feature dependencies.
    pub const fn windows_features(&self) -> &BTreeSet<String> {
        &self.windows_features
    }

    /// Returns the windows library dependencies.
    pub const fn windows_libraries(&self) -> &BTreeSet<String> {
        &self.windows_libraries
    }

    /// Returns the package dependencies.
    pub const fn packages(&self) -> &BTreeSet<PackageDependency> {
        &self.packages
    }

    /// Returns the external dependencies.
    pub const fn external(&self) -> &BTreeSet<String> {
        &self.external
    }

    /// Returns `true` if all the dependency fields are empty.
    ///
    /// # Examples
    /// ```
    /// # use std::collections::BTreeSet;
    /// # use winget_types::installer::{Dependencies, PackageDependency};
    /// # use winget_types::{PackageIdentifier, PackageIdentifierError};
    /// # fn main() -> Result<(), PackageIdentifierError> {
    /// let mut dependencies = Dependencies::default();
    ///
    /// assert!(dependencies.is_empty());
    ///
    /// let git = PackageIdentifier::new("Git.Git")?;
    /// dependencies.packages.insert(PackageDependency::new(git));
    ///
    /// assert!(!dependencies.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.windows_features.is_empty()
            && self.windows_libraries.is_empty()
            && self.packages.is_empty()
            && self.external.is_empty()
    }
}

#[cfg(all(feature = "serde", test))]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    use super::Dependencies;
    use crate::PackageIdentifier;

    #[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Manifest {
        dependencies: Dependencies,
    }

    #[rstest]
    #[case(
        Manifest {
            dependencies: Dependencies::builder()
                .packages([PackageIdentifier::new("Microsoft.VCRedist.2015+.x64").unwrap().into()])
                .build(),
        },
        indoc! {"
            Dependencies:
              PackageDependencies:
              - PackageIdentifier: Microsoft.VCRedist.2015+.x64
        "}
    )]
    fn serialize(#[case] manifest: Manifest, #[case] manifest_str: &str) {
        assert_eq!(
            serde_yaml::to_string(&manifest).as_deref().map_err(|_| ()),
            Ok(manifest_str)
        );
    }
}

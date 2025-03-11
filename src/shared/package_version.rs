use core::{fmt, str::FromStr};

use thiserror::Error;

use super::{DISALLOWED_CHARACTERS, version::Version};

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Version"))]
#[repr(transparent)]
pub struct PackageVersion(Version);

#[derive(Error, Debug, Eq, PartialEq)]
pub enum PackageVersionError {
    #[error("Package version contains invalid character {_0:?}")]
    InvalidCharacter(char),
    #[error("Package version cannot be empty")]
    Empty,
    #[error(
        "Package version cannot be more than {} characters long",
        PackageVersion::MAX_CHAR_LENGTH
    )]
    TooLong,
}

impl PackageVersion {
    const MAX_CHAR_LENGTH: usize = 128;

    /// Creates a new `PackageVersion` from any type that implements `AsRef<str>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the `PackageVersion` is empty, more than 128 characters long, or
    /// contains a disallowed character (control or one of [`DISALLOWED_CHARACTERS`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::PackageVersion;
    /// # use winget_types::PackageVersionError;
    ///
    /// # fn main() -> Result<(), PackageVersionError>  {
    /// let version = PackageVersion::new("1.2.3")?;
    /// let other_version = PackageVersion::new("1.2.4.0")?;
    ///
    /// assert!(version < other_version);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str>>(version: T) -> Result<Self, PackageVersionError> {
        let version = version.as_ref();

        if version.is_empty() {
            return Err(PackageVersionError::Empty);
        }

        let char_count = version.chars().try_fold(0, |char_count, char| {
            if DISALLOWED_CHARACTERS.contains(&char) || char.is_control() {
                return Err(PackageVersionError::InvalidCharacter(char));
            }

            Ok(char_count + 1)
        })?;

        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(PackageVersionError::TooLong);
        }

        Ok(Self(Version::new(version)))
    }

    /// Creates a new `PackageVersion` from any type that implements `AsRef<str>`, without checking
    /// its validity.
    ///
    /// # Safety
    ///
    /// The package version must not be more than 128 characters long, or contain a disallowed
    /// character (control, or one of [`DISALLOWED_CHARACTERS`]).
    #[inline]
    pub unsafe fn new_unchecked<T: AsRef<str>>(version: T) -> Self {
        Self(Version::new(version))
    }

    /// Returns true if the version matches `latest` (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::PackageVersion;
    /// # use winget_types::PackageVersionError;
    ///
    /// # fn main() -> Result<(), PackageVersionError> {
    /// assert!(PackageVersion::new("latest")?.is_latest());
    /// assert!(PackageVersion::new("LATEST")?.is_latest());
    /// assert!(!PackageVersion::new("1.2.3")?.is_latest());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    #[inline]
    pub fn is_latest(&self) -> bool {
        self.0.is_latest()
    }

    /// Extracts a string slice containing the entire `PackageVersion`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Extracts the inner `Version`.
    #[must_use]
    #[inline]
    pub const fn inner(&self) -> &Version {
        &self.0
    }

    /// Finds the closest version to this version from a given list of package versions.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::PackageVersion;
    /// # use winget_types::PackageVersionError;
    ///
    /// # fn main() -> Result<(), PackageVersionError> {
    /// let versions = [PackageVersion::new("1.2.5")?, PackageVersion::new("1.2.0")?];
    ///
    /// let version = PackageVersion::new("1.2.3")?;
    ///
    /// assert_eq!(version.closest(&versions).map(PackageVersion::as_str), Some("1.2.5"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn closest<'iter, I>(&self, versions: I) -> Option<&'iter Self>
    where
        I: IntoIterator<Item = &'iter Self>,
    {
        self.0.closest(versions)
    }
}

impl fmt::Display for PackageVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PackageVersion {
    type Err = PackageVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl<'ver> From<&'ver PackageVersion> for &'ver Version {
    #[inline]
    fn from(value: &'ver PackageVersion) -> Self {
        &value.0
    }
}

impl TryFrom<Version> for PackageVersion {
    type Error = PackageVersionError;

    #[inline]
    fn try_from(value: Version) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl PartialEq<Version> for PackageVersion {
    fn eq(&self, other: &Version) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<PackageVersion> for Version {
    fn eq(&self, other: &PackageVersion) -> bool {
        self.eq(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::{DISALLOWED_CHARACTERS, PackageVersion, PackageVersionError};

    #[test]
    fn empty_package_version() {
        assert_eq!(PackageVersion::new(""), Err(PackageVersionError::Empty));
    }

    #[test]
    fn disallowed_characters_in_package_version() {
        for char in DISALLOWED_CHARACTERS {
            assert_eq!(
                format!("1.2{char}3").parse::<PackageVersion>(),
                Err(PackageVersionError::InvalidCharacter(char))
            )
        }
    }

    #[test]
    fn control_characters_in_package_version() {
        assert_eq!(
            "1.2\03".parse::<PackageVersion>(),
            Err(PackageVersionError::InvalidCharacter('\0'))
        );
    }

    #[test]
    fn unicode_package_version_max_length() {
        let version = "🦀".repeat(PackageVersion::MAX_CHAR_LENGTH);

        // Ensure that it's character length that's being checked and not byte or UTF-16 length
        assert!(version.len() > PackageVersion::MAX_CHAR_LENGTH);
        assert!(version.encode_utf16().count() > PackageVersion::MAX_CHAR_LENGTH);
        assert_eq!(version.chars().count(), PackageVersion::MAX_CHAR_LENGTH);
        assert!(PackageVersion::new(version).is_ok());
    }

    #[test]
    fn package_version_too_long() {
        let version = "🦀".repeat(PackageVersion::MAX_CHAR_LENGTH + 1);

        assert_eq!(
            version.parse::<PackageVersion>(),
            Err(PackageVersionError::TooLong)
        );
    }
}

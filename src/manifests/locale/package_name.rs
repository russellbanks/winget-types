use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct PackageName(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PackageNameError {
    #[error(
        "Package name must have at least {} characters but has {_0}",
        PackageName::MIN_CHAR_LENGTH
    )]
    TooShort(usize),
    #[error(
        "Package name must not have more than {} characters but has {_0}",
        PackageName::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl PackageName {
    pub const MIN_CHAR_LENGTH: usize = 2;
    pub const MAX_CHAR_LENGTH: usize = 256;

    /// Creates a new `PackageName` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the package name is less than 2 characters long or more than 256
    /// characters long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::locale::PackageName;
    /// # use winget_types::locale::PackageNameError;
    ///
    /// # fn main() -> Result<(), PackageNameError>  {
    /// let package_name = PackageName::new("PowerShell")?;
    ///
    /// assert_eq!(package_name.as_str(), "PowerShell");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(
        package_name: T,
    ) -> Result<Self, PackageNameError> {
        match package_name.as_ref().chars().count() {
            count if count < Self::MIN_CHAR_LENGTH => Err(PackageNameError::TooShort(count)),
            count if count > Self::MAX_CHAR_LENGTH => Err(PackageNameError::TooLong(count)),
            _ => Ok(Self(package_name.into())),
        }
    }

    /// Creates a new `PackageName` from any type that implements `Into<CompactString>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The package name must not be less than 2 characters long or more than 256 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(package_name: T) -> Self {
        Self(package_name.into())
    }

    /// Extracts a string slice containing the entire `PackageName`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for PackageName {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for PackageName {
    fn default() -> Self {
        // SAFETY: `Package name` is longer than 2 characters
        unsafe { Self::new_unchecked("Package name") }
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PackageName {
    type Err = PackageNameError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for PackageName {
    type Error = PackageNameError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

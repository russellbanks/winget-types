use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct PortableCommandAlias(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PortableCommandAliasError {
    #[error("Portable command alias must not be empty")]
    Empty,
    #[error(
        "Portable command alias must not have more than {} characters but has {_0}",
        PortableCommandAlias::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl PortableCommandAlias {
    pub const MAX_CHAR_LENGTH: usize = 40;

    /// Creates a new `PortableCommandAlias` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the portable command alias is empty or more than 40 characters long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::PortableCommandAlias;
    /// # use winget_types::installer::PortableCommandAliasError;
    ///
    /// # fn main() -> Result<(), PortableCommandAliasError>  {
    /// let command_alias = PortableCommandAlias::new("ffmpeg")?;
    ///
    /// assert_eq!(command_alias.as_str(), "ffmpeg");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(
        command_alias: T,
    ) -> Result<Self, PortableCommandAliasError> {
        let alias = command_alias.as_ref().trim();

        if alias.is_empty() {
            return Err(PortableCommandAliasError::Empty);
        }

        let char_count = alias.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(PortableCommandAliasError::TooLong(char_count));
        }

        Ok(Self(command_alias.into()))
    }

    /// Creates a new `PortableCommandAlias` from any type that implements `Into<CompactString>`
    /// without checking its validity.
    ///
    /// # Safety
    ///
    /// The command must not be empty or more than 40 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(alias: T) -> Self {
        Self(alias.into())
    }

    /// Extracts a string slice containing the entire `PortableCommandAlias`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for PortableCommandAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PortableCommandAlias {
    type Err = PortableCommandAliasError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for PortableCommandAlias {
    type Error = PortableCommandAliasError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

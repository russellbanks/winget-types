use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct Copyright(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CopyrightError {
    #[error(
        "Copyright must have at least {} characters but has {_0}",
        Copyright::MIN_CHAR_LENGTH
    )]
    TooShort(usize),
    #[error(
        "Copyright must not have more than {} characters but has {_0}",
        Copyright::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Copyright {
    pub const MIN_CHAR_LENGTH: usize = 3;
    pub const MAX_CHAR_LENGTH: usize = 512;

    /// Creates a new `Copyright` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the copyright is less than 3 characters long or more than 512 characters
    /// long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::locale::Copyright;
    /// # use winget_types::locale::CopyrightError;
    ///
    /// # fn main() -> Result<(), CopyrightError>  {
    /// let copyright = Copyright::new("Copyright © Company")?;
    ///
    /// assert_eq!(copyright.as_str(), "Copyright © Company");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(copyright: T) -> Result<Self, CopyrightError> {
        match copyright.as_ref().chars().count() {
            count if count < Self::MIN_CHAR_LENGTH => Err(CopyrightError::TooShort(count)),
            count if count > Self::MAX_CHAR_LENGTH => Err(CopyrightError::TooLong(count)),
            _ => Ok(Self(copyright.into())),
        }
    }

    /// Creates a new `Copyright` from any type that implements `Into<CompactString>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The license must not be less than 3 characters long or more than 512 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(copyright: T) -> Self {
        Self(copyright.into())
    }

    /// Extracts a string slice containing the entire `Copyright`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Copyright {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Copyright {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Copyright {
    type Err = CopyrightError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for Copyright {
    type Error = CopyrightError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

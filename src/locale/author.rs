use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct Author(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AuthorError {
    #[error(
        "Author must have at least {} characters but has {_0}",
        Author::MIN_CHAR_LENGTH
    )]
    TooShort(usize),
    #[error(
        "Author must not have more than {} characters but has {_0}",
        Author::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Author {
    pub const MIN_CHAR_LENGTH: usize = 2;
    pub const MAX_CHAR_LENGTH: usize = 256;

    /// Creates a new `Author` from any type that implements `AsRef<str>` and `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the author is less than 2 characters long or more than 256 characters
    /// long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::locale::Author;
    /// # use winget_types::locale::AuthorError;
    ///
    /// # fn main() -> Result<(), AuthorError>  {
    /// let license = Author::new("John Smith")?;
    ///
    /// assert_eq!(license.as_str(), "John Smith");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(author: T) -> Result<Self, AuthorError> {
        match author.as_ref().chars().count() {
            count if count < Self::MIN_CHAR_LENGTH => Err(AuthorError::TooShort(count)),
            count if count > Self::MAX_CHAR_LENGTH => Err(AuthorError::TooLong(count)),
            _ => Ok(Self(author.into())),
        }
    }

    /// Creates a new `Author` from any type that implements `Into<CompactString>` without checking
    /// its validity.
    ///
    /// # Safety
    ///
    /// The author must not be less than 2 characters long or more than 256 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(author: T) -> Self {
        Self(author.into())
    }

    /// Extracts a string slice containing the entire `Author`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Author {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Author {
    type Err = AuthorError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for Author {
    type Error = AuthorError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

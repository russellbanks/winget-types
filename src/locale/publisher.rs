use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct Publisher(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PublisherError {
    #[error(
        "Publisher must have at least {} characters but has {_0}",
        Publisher::MIN_CHAR_LENGTH
    )]
    TooShort(usize),
    #[error(
        "Publisher must not have more than {} characters but has {_0}",
        Publisher::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Publisher {
    pub const MIN_CHAR_LENGTH: usize = 2;

    pub const MAX_CHAR_LENGTH: usize = 256;

    /// Creates a new `Publisher` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the publisher is less than 2 characters long or more than 256 characters
    /// long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::locale::Publisher;
    /// # use winget_types::locale::PublisherError;
    ///
    /// # fn main() -> Result<(), PublisherError>  {
    /// let publisher = Publisher::new("Microsoft")?;
    ///
    /// assert_eq!(publisher.as_str(), "Microsoft");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(publisher: T) -> Result<Self, PublisherError> {
        match publisher.as_ref().chars().count() {
            count if count < Self::MIN_CHAR_LENGTH => Err(PublisherError::TooShort(count)),
            count if count > Self::MAX_CHAR_LENGTH => Err(PublisherError::TooLong(count)),
            _ => Ok(Self(publisher.into())),
        }
    }

    /// Creates a new `Publisher` from any type that implements `Into<CompactString>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The publisher must not be less than 2 characters long or more than 256 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(publisher: T) -> Self {
        Self(publisher.into())
    }

    /// Extracts a string slice containing the entire `Publisher`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Publisher {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for Publisher {
    fn default() -> Self {
        // SAFETY: `Publisher` is longer than 2 characters
        unsafe { Self::new_unchecked("Publisher") }
    }
}

impl fmt::Display for Publisher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Publisher {
    type Err = PublisherError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for Publisher {
    type Error = PublisherError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

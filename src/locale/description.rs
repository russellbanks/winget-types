use alloc::string::String;
use core::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct Description(String);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum DescriptionError {
    #[error(
        "Description must have at least {} characters but has {_0}",
        Description::MIN_CHAR_LENGTH
    )]
    TooShort(usize),
    #[error(
        "Description must not have more than {} characters but has {_0}",
        Description::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Description {
    pub const MIN_CHAR_LENGTH: usize = 3;

    pub const MAX_CHAR_LENGTH: usize = 10_000;

    /// Creates a new `Description` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the description is less than 3 characters long or more than 10,000
    /// characters long.
    pub fn new<T: AsRef<str> + Into<String>>(description: T) -> Result<Self, DescriptionError> {
        match description.as_ref().chars().count() {
            count if count < Self::MIN_CHAR_LENGTH => Err(DescriptionError::TooShort(count)),
            count if count > Self::MAX_CHAR_LENGTH => Err(DescriptionError::TooLong(count)),
            _ => Ok(Self(description.into())),
        }
    }

    /// Creates a new `Description` from any type that implements `Into<String>` without checking
    /// its validity.
    ///
    /// # Safety
    ///
    /// The description must not be less than 3 characters long or more than 10,000 characters long.
    #[inline]
    pub unsafe fn new_unchecked<T: Into<String>>(description: T) -> Self {
        Self(description.into())
    }

    /// Extracts a string slice containing the entire `Description`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Description {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Description {
    type Err = DescriptionError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for Description {
    type Error = DescriptionError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

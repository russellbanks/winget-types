use alloc::string::String;
use core::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct ShortDescription(String);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ShortDescriptionError {
    #[error(
        "Short description must have at least {} characters but has {_0}",
        ShortDescription::MIN_CHAR_LENGTH
    )]
    TooShort(usize),
    #[error(
        "Short description must not have more than {} characters but has {_0}",
        ShortDescription::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl ShortDescription {
    pub const MIN_CHAR_LENGTH: usize = 2;
    pub const MAX_CHAR_LENGTH: usize = 256;

    /// Creates a new `ShortDescription` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the command is less than 2 characters long or more than 256 characters
    /// long.
    pub fn new<T: AsRef<str> + Into<String>>(
        short_description: T,
    ) -> Result<Self, ShortDescriptionError> {
        match short_description.as_ref().chars().count() {
            count if count < Self::MIN_CHAR_LENGTH => Err(ShortDescriptionError::TooShort(count)),
            count if count > Self::MAX_CHAR_LENGTH => Err(ShortDescriptionError::TooLong(count)),
            _ => Ok(Self(short_description.into())),
        }
    }

    /// Creates a new `ShortDescription` from any type that implements `Into<String>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The short description must not be less than 2 characters long or more than 256 characters
    /// long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<String>>(short_description: T) -> Self {
        Self(short_description.into())
    }

    /// Extracts a string slice containing the entire `ShortDescription`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for ShortDescription {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for ShortDescription {
    fn default() -> Self {
        // SAFETY: `Short description` is greater than 3 characters
        unsafe { Self::new_unchecked("Short description") }
    }
}

impl fmt::Display for ShortDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ShortDescription {
    type Err = ShortDescriptionError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for ShortDescription {
    type Error = ShortDescriptionError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

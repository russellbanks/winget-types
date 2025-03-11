use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

use crate::DISALLOWED_CHARACTERS;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct FileExtension(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum FileExtensionError {
    #[error("File extension contains invalid character {_0:?}")]
    InvalidCharacter(char),
    #[error("File extension must not be empty")]
    Empty,
    #[error(
        "File extension must not have more than {} characters but has {_0}",
        FileExtension::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl FileExtension {
    pub const MAX_CHAR_LENGTH: usize = 64;

    /// Creates a new `FileExtension` from any type that implements `AsRef<str>`.
    ///
    /// Leading dots (`.`) are trimmed.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the file extension is empty, more than 64 characters long, or contains a
    /// disallowed character (control or one of [`DISALLOWED_CHARACTERS`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::FileExtension;
    /// # use winget_types::installer::FileExtensionError;
    ///
    /// # fn main() -> Result<(), FileExtensionError>  {
    /// let extension = FileExtension::new("xml")?;
    ///
    /// assert_eq!(extension.as_str(), "xml");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str>>(file_extension: T) -> Result<Self, FileExtensionError> {
        let extension = file_extension.as_ref().trim_start_matches('.');

        if extension.is_empty() {
            return Err(FileExtensionError::Empty);
        }

        let char_count = extension.chars().try_fold(0, |char_count, char| {
            if DISALLOWED_CHARACTERS.contains(&char) || char.is_control() {
                return Err(FileExtensionError::InvalidCharacter(char));
            }

            Ok(char_count + 1)
        })?;

        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(FileExtensionError::TooLong(char_count));
        }

        Ok(Self(extension.into()))
    }

    /// Creates a new `FileExtension` from any type that implements `Into<CompactString>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The file extension must not be empty, be more than 64 characters long, or contain a
    /// disallowed character (control or one of [`DISALLOWED_CHARACTERS`]).
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(file_extension: T) -> Self {
        Self(file_extension.into())
    }

    /// Extracts a string slice containing the entire `FileExtension`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for FileExtension {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FileExtension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for FileExtension {
    type Err = FileExtensionError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for FileExtension {
    type Error = FileExtensionError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

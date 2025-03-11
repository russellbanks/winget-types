use alloc::string::String;
use core::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct InstallationNotes(String);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum InstallationNotesError {
    #[error("Installation notes must not be empty")]
    Empty,
    #[error(
        "Installation notes must not have more than {} characters but has {_0}",
        InstallationNotes::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl InstallationNotes {
    pub const MAX_CHAR_LENGTH: usize = 10_000;

    /// Creates a new `InstallationNotes` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the installation notes are empty or more than 10,000 characters long.
    pub fn new<T: AsRef<str> + Into<String>>(
        installation_notes: T,
    ) -> Result<Self, InstallationNotesError> {
        let notes = installation_notes.as_ref();

        if notes.is_empty() {
            return Err(InstallationNotesError::Empty);
        }

        let char_count = notes.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(InstallationNotesError::TooLong(char_count));
        }

        Ok(Self(installation_notes.into()))
    }

    /// Creates a new `InstallationNotes` from any type that implements `Into<String>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The installation notes must not be empty or more than 10,000 characters long.
    #[inline]
    pub unsafe fn new_unchecked<T: Into<String>>(installation_notes: T) -> Self {
        Self(installation_notes.into())
    }

    /// Extracts a string slice containing the entire `InstallationNotes`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for InstallationNotes {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for InstallationNotes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for InstallationNotes {
    type Err = InstallationNotesError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for InstallationNotes {
    type Error = InstallationNotesError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::locale::installation_notes::{InstallationNotes, InstallationNotesError};

    #[test]
    fn valid_installation_notes() {
        assert!(
            "Be careful when using this application"
                .parse::<InstallationNotes>()
                .is_ok()
        );
    }

    #[test]
    fn installation_notes_max_length() {
        let installation_notes = "🦀".repeat(InstallationNotes::MAX_CHAR_LENGTH);

        // Ensure that it's character length that's being checked and not byte or UTF-16 length
        assert!(installation_notes.len() > InstallationNotes::MAX_CHAR_LENGTH);
        assert!(installation_notes.encode_utf16().count() > InstallationNotes::MAX_CHAR_LENGTH);
        assert_eq!(
            installation_notes.chars().count(),
            InstallationNotes::MAX_CHAR_LENGTH
        );
        assert!(InstallationNotes::new(installation_notes).is_ok());
    }

    #[test]
    fn installation_notes_too_long() {
        let installation_notes = "a".repeat(InstallationNotes::MAX_CHAR_LENGTH + 1);

        assert_eq!(
            installation_notes.parse::<InstallationNotes>(),
            Err(InstallationNotesError::TooLong(installation_notes.len()))
        );
    }

    #[test]
    fn empty_installation_notes() {
        assert_eq!(
            "".parse::<InstallationNotes>(),
            Err(InstallationNotesError::Empty)
        );
    }
}

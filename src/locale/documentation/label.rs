use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct DocumentLabel(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum DocumentLabelError {
    #[error("Document label must not be empty")]
    Empty,
    #[error(
        "Document label must not have more than {} characters but has {_0}",
        DocumentLabel::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl DocumentLabel {
    pub const MAX_CHAR_LENGTH: usize = 100;

    /// Creates a new `DocumentLabel` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the document label is empty or more than 100 characters long.
    pub fn new<T: AsRef<str> + Into<CompactString>>(
        document_label: T,
    ) -> Result<Self, DocumentLabelError> {
        let channel_str = document_label.as_ref();

        if channel_str.is_empty() {
            return Err(DocumentLabelError::Empty);
        }

        let char_count = channel_str.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(DocumentLabelError::TooLong(char_count));
        }

        Ok(Self(document_label.into()))
    }

    /// Creates a new `DocumentLabel` from any type that implements `Into<CompactString>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The document label must not be empty or more than 100 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(document_label: T) -> Self {
        Self(document_label.into())
    }

    /// Extracts a string slice containing the entire `DocumentLabel`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for DocumentLabel {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DocumentLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for DocumentLabel {
    type Err = DocumentLabelError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for DocumentLabel {
    type Error = DocumentLabelError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

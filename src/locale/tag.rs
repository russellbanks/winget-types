use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct Tag(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TagError {
    #[error("Tag must not be empty")]
    Empty,
    #[error(
        "Tag must not have more than {} characters but has {_0}",
        Tag::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Tag {
    pub const MAX_CHAR_LENGTH: usize = 40;

    /// Creates a new `Tag` from any type that implements `AsRef<str>` and `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the tag is empty or more than 40 characters long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::locale::Tag;
    /// # use winget_types::locale::TagError;
    ///
    /// # fn main() -> Result<(), TagError>  {
    /// let tag = Tag::new("winget")?;
    ///
    /// assert_eq!(tag.as_str(), "winget");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(tag: T) -> Result<Self, TagError> {
        let tag_str = tag.as_ref();

        if tag_str.is_empty() {
            return Err(TagError::Empty);
        }

        let char_count = tag_str.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(TagError::TooLong(char_count));
        }

        Ok(Self(tag.into()))
    }

    /// Creates a new `Tag` from any type that implements `Into<CompactString>` without checking
    /// its validity.
    ///
    /// # Safety
    ///
    /// The tag must not be empty or more than 40 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(tag: T) -> Self {
        Self(tag.into())
    }

    /// Extracts a string slice containing the entire `Tag`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Tag {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Tag {
    type Err = TagError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for Tag {
    type Error = TagError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

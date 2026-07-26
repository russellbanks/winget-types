use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct Resource(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ResourceError {
    #[error("Resource must not be empty")]
    Empty,
    #[error(
        "Resource must not have more than {} characters but has {_0}",
        Resource::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Resource {
    pub const MAX_CHAR_LENGTH: usize = 512;

    /// Creates a new `Resource` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the resource is empty or more than 512 characters long.
    pub fn new<T: AsRef<str> + Into<CompactString>>(resource: T) -> Result<Self, ResourceError> {
        let resource_str = resource.as_ref();

        if resource_str.is_empty() {
            return Err(ResourceError::Empty);
        }

        let char_count = resource_str.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(ResourceError::TooLong(char_count));
        }

        Ok(Self(resource.into()))
    }

    /// Creates a new `Resource` from any type that implements `Into<CompactString>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The resource must not be empty or more than 512 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(resource: T) -> Self {
        Self(resource.into())
    }

    /// Extracts a string slice containing the entire `Resource`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Resource {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Resource {
    type Err = ResourceError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for Resource {
    type Error = ResourceError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

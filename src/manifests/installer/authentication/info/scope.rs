use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct Scope(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ScopeError {
    #[error("Scope must not be empty")]
    Empty,
    #[error(
        "Scope must not have more than {} characters but has {_0}",
        Scope::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Scope {
    pub const MAX_CHAR_LENGTH: usize = 512;

    /// Creates a new `Scope` from any type that implements `AsRef<str>` and `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the scope is empty or more than 512 characters long.
    pub fn new<T: AsRef<str> + Into<CompactString>>(scope: T) -> Result<Self, ScopeError> {
        let scope_str = scope.as_ref();

        if scope_str.is_empty() {
            return Err(ScopeError::Empty);
        }

        let char_count = scope_str.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(ScopeError::TooLong(char_count));
        }

        Ok(Self(scope.into()))
    }

    /// Creates a new `Scope` from any type that implements `Into<CompactString>` without checking
    /// its validity.
    ///
    /// # Safety
    ///
    /// The scope must not be empty or more than 512 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(scope: T) -> Self {
        Self(scope.into())
    }

    /// Extracts a string slice containing the entire `Scope`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Scope {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Scope {
    type Err = ScopeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for Scope {
    type Error = ScopeError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

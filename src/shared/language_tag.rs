use core::{cmp::Ordering, fmt, str::FromStr};

use icu_locid::{LanguageIdentifier, ParserError, langid};

/// An orderable wrapper around [`LanguageIdentifier`].
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct LanguageTag(LanguageIdentifier);

impl LanguageTag {
    #[must_use]
    #[inline]
    pub const fn new(language: LanguageIdentifier) -> Self {
        Self(language)
    }
}

impl Default for LanguageTag {
    fn default() -> Self {
        Self(langid!("en-US"))
    }
}

impl fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for LanguageTag {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        LanguageIdentifier::from_str(source).map(Self)
    }
}

impl PartialOrd for LanguageTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LanguageTag {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.total_cmp(&self.0)
    }
}

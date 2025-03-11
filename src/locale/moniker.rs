use core::{fmt, ops::Deref, str::FromStr};

use super::{Tag, TagError};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Moniker(Tag);

impl Deref for Moniker {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Moniker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Moniker {
    type Err = TagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Tag::from_str(s).map(Self)
    }
}

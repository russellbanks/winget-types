use core::{fmt, str::FromStr};

use super::DecodedUrl;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CopyrightUrl(DecodedUrl);

impl CopyrightUrl {
    /// Returns the serialization of this URL.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for CopyrightUrl {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for CopyrightUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for CopyrightUrl {
    type Err = <DecodedUrl as FromStr>::Err;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        DecodedUrl::from_str(src).map(Self)
    }
}

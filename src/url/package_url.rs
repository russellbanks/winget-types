use core::{fmt, str::FromStr};

use url::ParseError;

use super::DecodedUrl;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PackageUrl(DecodedUrl);

impl PackageUrl {
    /// Returns the serialization of this URL.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for PackageUrl {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for PackageUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PackageUrl {
    type Err = ParseError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        DecodedUrl::from_str(src).map(Self)
    }
}

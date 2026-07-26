use core::{fmt, str::FromStr};

use super::DecodedUrl;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PublisherUrl(DecodedUrl);

impl PublisherUrl {
    /// Returns the serialization of this URL.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for PublisherUrl {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for PublisherUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PublisherUrl {
    type Err = <DecodedUrl as FromStr>::Err;

    fn from_str(src: &str) -> Result<Self, <DecodedUrl as FromStr>::Err> {
        DecodedUrl::from_str(src).map(Self)
    }
}

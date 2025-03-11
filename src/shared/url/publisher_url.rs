use core::{fmt, str::FromStr};

use super::DecodedUrl;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PublisherUrl(DecodedUrl);

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

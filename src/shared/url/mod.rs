mod copyright_url;
mod license_url;
mod package_url;
mod publisher_support_url;
mod publisher_url;
mod release_notes_url;

use core::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub use copyright_url::CopyrightUrl;
pub use license_url::LicenseUrl;
pub use package_url::PackageUrl;
use percent_encoding::percent_decode_str;
pub use publisher_support_url::PublisherSupportUrl;
pub use publisher_url::PublisherUrl;
pub use release_notes_url::ReleaseNotesUrl;
use url::{ParseError, Url};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecodedUrl(Url);

impl DecodedUrl {
    /// Returns the serialization of this URL.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for DecodedUrl {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Default for DecodedUrl {
    fn default() -> Self {
        Self(Url::parse("https://example.com").unwrap_or_else(|_| unreachable!()))
    }
}

impl Deref for DecodedUrl {
    type Target = Url;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DecodedUrl {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for DecodedUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for DecodedUrl {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::parse(&percent_decode_str(s).decode_utf8_lossy()).map(Self)
    }
}

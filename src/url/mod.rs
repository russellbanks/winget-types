mod copyright_url;
mod license_url;
mod package_url;
mod publisher_support_url;
mod publisher_url;
mod release_notes_url;

use alloc::{borrow::Cow, string::String};
use core::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use compact_str::CompactString;
pub use copyright_url::CopyrightUrl;
pub use license_url::LicenseUrl;
pub use package_url::PackageUrl;
pub use publisher_support_url::PublisherSupportUrl;
pub use publisher_url::PublisherUrl;
pub use release_notes_url::ReleaseNotesUrl;
use url::{ParseError, Url};

/// A URL whose path has redundant percent-escapes decoded.
///
/// An escape in the path is decoded only when the decoded character means the same thing as the
/// escape, for example `%2B` becomes `+` and `%28` becomes `(`. Escapes whose decoding would change
/// the URL, such as `%2F` (`/`), `%3F` (`?`), `%23` (`#`), `%25` (`%`) or `%20` (space), are kept.
/// The query and fragment are left as they are.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
pub struct DecodedUrl(Url);

/// Returns true if a byte means the same thing in a URL path whether it is percent-encoded or not.
///
/// This is the RFC 3986 `pchar` set without `%` and the characters the `url` crate re-encodes.
const fn is_redundantly_encoded(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'-' | b'.'
                | b'_'
                | b'~'
                | b'!'
                | b'$'
                | b'&'
                | b'\''
                | b'('
                | b')'
                | b'*'
                | b'+'
                | b','
                | b';'
                | b'='
                | b':'
                | b'@'
        )
}

/// Decodes the percent-escapes in a URL path that are [redundantly encoded].
///
/// [redundantly encoded]: is_redundantly_encoded
fn decode_redundant_escapes(path: &str) -> Cow<'_, str> {
    fn hex_digit(byte: Option<&u8>) -> Option<u8> {
        char::from(*byte?)
            .to_digit(16)
            .and_then(|digit| u8::try_from(digit).ok())
    }

    let bytes = path.as_bytes();
    let mut decoded = String::new();
    let mut copied_up_to = 0;
    let mut index = 0;

    while let Some(offset) = bytes[index..].iter().position(|&byte| byte == b'%') {
        let start = index + offset;

        let escaped = hex_digit(bytes.get(start + 1))
            .zip(hex_digit(bytes.get(start + 2)))
            .map(|(high, low)| high << 4 | low)
            .filter(|&byte| is_redundantly_encoded(byte));

        if let Some(byte) = escaped {
            decoded.push_str(&path[copied_up_to..start]);
            decoded.push(char::from(byte));
            copied_up_to = start + 3;
            index = start + 3;
        } else {
            index = start + 1;
        }
    }

    if copied_up_to == 0 {
        Cow::Borrowed(path)
    } else {
        decoded.push_str(&path[copied_up_to..]);
        Cow::Owned(decoded)
    }
}

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
        let mut url = Url::parse(s)?;

        if let Cow::Owned(path) = decode_redundant_escapes(url.path()) {
            url.set_path(&path);
        }

        Ok(Self(url))
    }
}

impl TryFrom<CompactString> for DecodedUrl {
    type Error = ParseError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::DecodedUrl;

    #[rstest]
    #[case::plus(
        "https://example.com/v1.0%2Bbuild/app.exe",
        "https://example.com/v1.0+build/app.exe"
    )]
    #[case::parentheses(
        "https://example.com/app%28x64%29.exe",
        "https://example.com/app(x64).exe"
    )]
    #[case::unreserved(
        "https://example.com/%7Euser/app%2Dv1.exe",
        "https://example.com/~user/app-v1.exe"
    )]
    fn decodes_safe_path_escapes(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(input.parse::<DecodedUrl>().unwrap().as_str(), expected);
    }

    #[rstest]
    #[case::hash("https://example.com/file%23v2.exe")]
    #[case::question_mark("https://example.com/file%3Fv2.exe")]
    #[case::slash("https://example.com/a%2Fb.exe")]
    #[case::percent("https://example.com/a%2541.exe")]
    #[case::space("https://example.com/my%20app.exe")]
    #[case::invalid_utf8("https://example.com/a%FF.exe")]
    #[case::query_delimiters("https://example.com/app.exe?a=b%26c%3Dd%2Be")]
    fn preserves_structural_escapes(#[case] input: &str) {
        assert_eq!(input.parse::<DecodedUrl>().unwrap().as_str(), input);
    }

    #[test]
    fn hash_escape_does_not_become_fragment() {
        let url = "https://example.com/file%23v2.exe"
            .parse::<DecodedUrl>()
            .unwrap();
        assert_eq!(url.fragment(), None);
        assert_eq!(url.path(), "/file%23v2.exe");
    }

    #[cfg(feature = "serde")]
    #[rstest]
    #[case("https://example.com/v1.0%2Bbuild/app.exe")]
    #[case("https://example.com/file%23v2.exe")]
    fn deserialize_matches_from_str(#[case] input: &str) {
        assert_eq!(
            serde_yaml::from_str::<DecodedUrl>(input).unwrap(),
            input.parse::<DecodedUrl>().unwrap()
        );
    }
}

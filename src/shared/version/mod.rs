mod part;

use alloc::{borrow::Cow, string::String};
use core::{
    cmp::{Ordering, Reverse},
    convert::Infallible,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use compact_str::CompactString;
use itertools::{EitherOrBoth, Itertools};
use part::VersionPart;
use smallvec::SmallVec;

#[derive(Clone, Debug, Default, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "&str"))]
pub struct Version {
    /// The original version string, used for display and serialization
    raw: CompactString,
    /// The split parts of a version, used for ordering and equality
    parts: SmallVec<[VersionPart; 6]>,
}

impl Version {
    const SEPARATOR: char = '.';

    pub fn new<T: AsRef<str>>(input: T) -> Self {
        let mut trimmed = input.as_ref().trim();

        // If there is a digit before the separator, or no separators, trim off all leading
        // non-digit characters
        if let Some(digit_pos) = trimmed.find(|char: char| char.is_ascii_digit()) {
            if trimmed
                .find('.')
                .is_none_or(|separator_pos| digit_pos < separator_pos)
            {
                trimmed = &trimmed[digit_pos..];
            }
        }

        // Split the version into parts by the separator `.`
        let mut parts = trimmed
            .split(Self::SEPARATOR)
            .map(VersionPart::from)
            .collect::<SmallVec<[_; 6]>>();

        // Remove all trailing `.0`
        if let Some(pos) = parts.iter().rposition(|part| !part.is_droppable()) {
            parts.truncate(pos + 1);
        } else {
            parts.clear();
        }

        Self {
            raw: CompactString::from(trimmed),
            parts,
        }
    }

    /// Returns true if the version matches `latest` (case-insensitive).
    ///
    /// The latest version is always the greatest of any versions.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::Version;
    ///
    /// assert!(Version::new("latest").is_latest());
    /// assert!(Version::new("LATEST").is_latest());
    /// assert!(!Version::new("1.2.3").is_latest());
    ///
    /// assert!(Version::new("latest") > Version::new("999.999.999"));
    /// ```
    #[must_use]
    #[inline]
    pub fn is_latest(&self) -> bool {
        const LATEST: &str = "latest";

        self.raw.eq_ignore_ascii_case(LATEST)
    }

    /// Returns true if the version matches `unknown` (case-insensitive).
    ///
    /// An unknown version is always the minimum of any versions.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::Version;
    ///
    /// assert!(Version::new("unknown").is_unknown());
    /// assert!(Version::new("UNKNOWN").is_unknown());
    /// assert!(!Version::new("1.2.3").is_unknown());
    ///
    /// assert!(Version::new("unknown") < Version::new("0"));
    /// ```
    #[must_use]
    #[inline]
    pub fn is_unknown(&self) -> bool {
        const UNKNOWN: &str = "unknown";

        self.raw.eq_ignore_ascii_case(UNKNOWN)
    }

    /// Extracts a string slice containing the entire `Version`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.raw.as_str()
    }

    /// Finds the closest version to this version from a given list of versions.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::Version;
    ///
    /// let versions = [Version::new("1.2.5"), Version::new("1.2.0")];
    ///
    /// let version = Version::new("1.2.3");
    ///
    /// assert_eq!(version.closest(&versions).map(Version::as_str), Some("1.2.5"));
    /// ```
    pub fn closest<'iter, I, T>(&self, versions: I) -> Option<&'iter T>
    where
        I: IntoIterator<Item = &'iter T>,
        &'iter T: Into<&'iter Self>,
    {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct DistanceKey<'supplement> {
            // Prefer versions that diverge later
            length_score: usize,
            // Prefer smaller numerical differences
            numerical_difference: u64,
            // Prefer higher versions
            total_order: Ordering,
            // Reverse order: prefer higher supplements lexicographically
            supplement_order: Reverse<&'supplement str>,
        }

        let default_part = &VersionPart::DEFAULT;

        // Find the version with the minimum 'distance'
        versions.into_iter().min_by_key(|&other| {
            self.parts
                .iter()
                .zip_longest(other.into().parts.iter())
                .map(|pair| match pair {
                    EitherOrBoth::Both(part, other_part) => (part, other_part),
                    EitherOrBoth::Left(part) => (part, default_part),
                    EitherOrBoth::Right(other_part) => (default_part, other_part),
                })
                .enumerate()
                .find_map(|(index, (part, other_part))| {
                    (part != other_part).then(|| DistanceKey {
                        length_score: !index,
                        numerical_difference: part.number.abs_diff(other_part.number),
                        total_order: part.cmp(other_part),
                        supplement_order: Reverse(other_part.supplement.as_str()),
                    })
                })
                .unwrap_or(DistanceKey {
                    length_score: 0,
                    numerical_difference: 0,
                    total_order: Ordering::Equal,
                    supplement_order: Reverse(""),
                })
        })
    }
}

impl AsRef<str> for Version {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl FromStr for Version {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl From<&str> for Version {
    #[inline]
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Version {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&String> for Version {
    #[inline]
    fn from(s: &String) -> Self {
        Self::new(s)
    }
}

impl From<Cow<'_, str>> for Version {
    #[inline]
    fn from(s: Cow<'_, str>) -> Self {
        Self::new(s)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        (self.is_latest() && other.is_latest())
            || (self.is_unknown() && other.is_unknown())
            || self.parts.eq(&other.parts)
    }
}

impl Hash for Version {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parts.hash(state);
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_latest(), other.is_latest()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => match (self.is_unknown(), other.is_unknown()) {
                (true, true) => Ordering::Equal,
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                (false, false) => self
                    .parts
                    .iter()
                    .zip_longest(&other.parts)
                    .map(|pair| match pair {
                        EitherOrBoth::Both(part, other_part) => part.cmp(other_part),
                        EitherOrBoth::Left(part) => part.cmp(&VersionPart::DEFAULT),
                        EitherOrBoth::Right(other_part) => VersionPart::DEFAULT.cmp(other_part),
                    })
                    .find(|&ordering| ordering != Ordering::Equal)
                    .unwrap_or(Ordering::Equal),
            },
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::cmp::Ordering;

    use rstest::rstest;

    use super::Version;

    #[rstest]
    #[case("1.0", "1.0.0")]
    #[case("1.2.00.3", "1.2.0.3")]
    #[case("1.2.003.4", "1.2.3.4")]
    #[case("01.02.03.04", "1.2.3.4")]
    #[case("1.2.03-beta", "1.2.3-beta")]
    #[case("1.0", "1.0 ")]
    #[case("1.0", "1. 0")]
    #[case("1.0", "1.0.")]
    #[case("1.0", "Version 1.0")]
    #[case("2.4.2", "v2.4.2")]
    #[case("foo1", "bar1")]
    #[case("latest", "LATEST")]
    #[case("unknown", "UNKNOWN")]
    fn version_equality(#[case] left: &str, #[case] right: &str) {
        let left = Version::new(left);
        let right = Version::new(right);
        assert_eq!(left, right);
        assert_eq!(left.cmp(&right), Ordering::Equal);
    }

    #[rstest]
    #[case("1", "2")]
    #[case("1.2-rc", "1.2")]
    #[case("1.0-rc", "1.0")]
    #[case("1.0.0-rc", "1")]
    #[case("22.0.0-rc.1", "22.0.0")]
    #[case("22.0.0-rc.1", "22.0.0.1")]
    #[case("22.0.0-rc.1", "22.0.0.1-rc")]
    #[case("22.0.0-rc.1", "22.0.0-rc.1.1")]
    #[case("22.0.0-rc.1.1", "22.0.0-rc.1.2")]
    #[case("22.0.0-rc.1.2", "22.0.0-rc.2")]
    #[case("v0.0.1", "0.0.2")]
    #[case("v0.0.1", "v0.0.2")]
    #[case("1.a2", "1.b1")]
    #[case("alpha", "beta")]
    #[case("99999.99999.99999", "latest")]
    #[case("unknown", "1.2.3")]
    #[case("unknown", "latest")]
    fn version_comparison_and_inequality(#[case] left: Version, #[case] right: Version) {
        assert!(left < right);
        assert!(right > left);
        assert_ne!(left, right)
    }

    #[rstest]
    #[case("1", "2")]
    #[case("1-rc", "1")]
    #[case("1-a2", "1-b1")]
    #[case("alpha", "beta")]
    fn version_part_comparison(#[case] left: Version, #[case] right: Version) {
        assert!(left < right);
        assert!(right > left);
    }

    #[test]
    fn version_hash() {
        use core::hash::BuildHasher;

        use rustc_hash::FxBuildHasher;

        // If two keys are equal, their hashes must also be equal
        // https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq

        let version1 = Version::new("1.2.3");
        let version2 = Version::new("1.2.3.0");
        assert_eq!(version1, version2);

        assert_eq!(
            FxBuildHasher.hash_one(version1),
            FxBuildHasher.hash_one(version2)
        );
    }

    #[test]
    fn only_supplement() {
        const ALPHA: &str = "alpha";

        let version = Version::new(ALPHA);
        assert_eq!(version.parts.len(), 1);
        assert_eq!(version.parts[0].number, 0);
        assert_eq!(version.parts[0].supplement, ALPHA);
    }

    #[rstest]
    #[case("0")]
    #[case("0.0.0")]
    #[case("0.0.0.0.0.0.0.0")]
    #[case("")]
    fn only_droppable_parts(#[case] version: Version) {
        assert_eq!(version.parts.len(), 0);
    }

    #[rstest]
    #[case("1.2.3", &["1.0.0", "0.9.0", "1.5.6.3", "1.3.2"], "1.3.2")]
    #[case("10.20.30", &["10.20.29", "10.20.31", "10.20.40"], "10.20.31")]
    #[case("5.5.5", &["5.5.50", "5.5.0", "5.5.10"], "5.5.10")]
    #[case("3.0.0", &["3.0.0-beta", "3.0.0-alpha.1", "3.0.0-rc.1"], "3.0.0-rc.1")]
    #[case("2.1.0-beta", &["2.1.0-alpha", "2.1.0-beta.2", "2.1.0"], "2.1.0-beta.2")]
    #[case("1.5.0", &["1.0.0", "2.0.0"], "1.0.0")]
    #[case("3.3.3", &["1.1.1", "5.5.5"], "5.5.5")]
    #[case("3.3.3", &["5.5.5", "1.1.1"], "5.5.5")]
    #[case("2.2.2", &["2.2.2", "2.2.2", "2.2.3"], "2.2.2")]
    #[case("0.0.2", &["0.0.1", "0.0.3", "0.2.0"], "0.0.3")]
    #[case("999.999.999", &["999.999.998", "1000.0.0"], "999.999.998")]
    fn closest_version(#[case] version: &str, #[case] versions: &[&str], #[case] expected: &str) {
        let versions = versions.into_iter().map(Version::new).collect::<Vec<_>>();
        assert_eq!(
            Version::new(version).closest(&versions),
            Some(&Version::new(expected))
        );
    }
}

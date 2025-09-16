use core::{fmt, num::ParseIntError, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
pub struct MinimumOSVersion(u16, u16, u16, u16);

#[derive(Error, Debug, Eq, PartialEq)]
pub enum MinimumOSVersionError {
    #[error("Minimum OS version must have at least a major version part")]
    NoVersionParts,
    #[error(transparent)]
    InvalidPart(#[from] ParseIntError),
}

impl MinimumOSVersion {
    const MAX_PARTS: usize = 4;
    const SEPARATOR: char = '.';

    /// Creates a new `MinimumOSVersion` from a `major`, `minor`, `patch`, and `build` part.
    #[must_use]
    #[inline]
    pub const fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self(major, minor, patch, build)
    }

    /// Returns the major version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use winget_types::installer::MinimumOSVersion;
    /// let minimum_os_version = MinimumOSVersion::new(10, 0, 17763, 0);
    /// assert_eq!(minimum_os_version.major(), 10);
    /// ```
    #[must_use]
    #[inline]
    pub const fn major(&self) -> u16 {
        self.0
    }

    /// Returns the minor version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use winget_types::installer::MinimumOSVersion;
    /// let minimum_os_version = MinimumOSVersion::new(1, 2, 3, 4);
    /// assert_eq!(minimum_os_version.minor(), 2);
    /// ```
    #[must_use]
    #[inline]
    pub const fn minor(&self) -> u16 {
        self.1
    }

    /// Returns the patch version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use winget_types::installer::MinimumOSVersion;
    /// let minimum_os_version = MinimumOSVersion::new(1, 2, 3, 4);
    /// assert_eq!(minimum_os_version.patch(), 3);
    /// ```
    #[must_use]
    #[inline]
    pub const fn patch(&self) -> u16 {
        self.2
    }

    /// Returns the build version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use winget_types::installer::MinimumOSVersion;
    /// let minimum_os_version = MinimumOSVersion::new(1, 2, 3, 4);
    /// assert_eq!(minimum_os_version.build(), 4);
    /// ```
    #[must_use]
    #[inline]
    pub const fn build(&self) -> u16 {
        self.3
    }
}

impl fmt::Display for MinimumOSVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

impl FromStr for MinimumOSVersion {
    type Err = MinimumOSVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(Self::MAX_PARTS, Self::SEPARATOR);

        let major = parts
            .next()
            .ok_or(MinimumOSVersionError::NoVersionParts)?
            .parse::<u16>()?;
        let minor = parts.next().map_or(Ok(0), str::parse::<u16>)?;
        let patch = parts.next().map_or(Ok(0), str::parse::<u16>)?;
        let build = parts.next().map_or(Ok(0), str::parse::<u16>)?;

        Ok(Self(major, minor, patch, build))
    }
}

impl From<(u16, u16, u16, u16)> for MinimumOSVersion {
    #[inline]
    fn from((major, minor, patch, build): (u16, u16, u16, u16)) -> Self {
        Self::new(major, minor, patch, build)
    }
}

impl TryFrom<CompactString> for MinimumOSVersion {
    type Error = MinimumOSVersionError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for MinimumOSVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use rstest::rstest;

    use super::MinimumOSVersion;

    #[rstest]
    #[case("10.0.17763.0", MinimumOSVersion(10, 0, 17763, 0))]
    #[case("11", MinimumOSVersion(11, 0, 0, 0))]
    #[case("10.1", MinimumOSVersion(10, 1, 0, 0))]
    #[case("0", MinimumOSVersion(0, 0, 0, 0))]
    #[case(
        "65535.65535.65535.65535",
        MinimumOSVersion(u16::MAX, u16::MAX, u16::MAX, u16::MAX)
    )]
    fn valid_minimum_os_version(
        #[case] minimum_os_version: &str,
        #[case] expected: MinimumOSVersion,
    ) {
        assert_eq!(
            minimum_os_version.parse::<MinimumOSVersion>().unwrap(),
            expected
        )
    }

    #[test]
    fn minimum_os_version_display() {
        let version = "1.2.3.4";

        assert_eq!(MinimumOSVersion(1, 2, 3, 4).to_string(), version);

        // Test round tripping
        assert_eq!(
            version.parse::<MinimumOSVersion>().unwrap().to_string(),
            version
        );
    }
}

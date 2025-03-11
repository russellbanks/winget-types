use core::{fmt, num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
pub struct ManifestVersion(u16, u16, u16);

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ManifestVersionError {
    #[error("Manifest version must have a major part")]
    NoMajorVersion,
    #[error("Manifest version must have a minor part")]
    NoMinorVersion,
    #[error("Manifest version must have a patch part")]
    NoPatchVersion,
    #[error(transparent)]
    InvalidPart(#[from] ParseIntError),
}

impl ManifestVersion {
    pub const DEFAULT: Self = Self(1, 10, 0);
    const PARTS_COUNT: u8 = 3;
    const SEPARATOR: char = '.';

    /// Creates a new `ManifestVersion` from a `major`, `minor`, and `patch` part.
    #[must_use]
    #[inline]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self(major, minor, patch)
    }
}

impl Default for ManifestVersion {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for ManifestVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl FromStr for ManifestVersion {
    type Err = ManifestVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(Self::PARTS_COUNT as usize, Self::SEPARATOR);

        let major = parts
            .next()
            .ok_or(ManifestVersionError::NoMajorVersion)?
            .parse::<u16>()?;
        let minor = parts
            .next()
            .ok_or(ManifestVersionError::NoMinorVersion)?
            .parse::<u16>()?;
        let patch = parts
            .next()
            .ok_or(ManifestVersionError::NoPatchVersion)?
            .parse::<u16>()?;

        Ok(Self(major, minor, patch))
    }
}

impl TryFrom<&str> for ManifestVersion {
    type Error = ManifestVersionError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ManifestVersion
where
    Self: fmt::Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}

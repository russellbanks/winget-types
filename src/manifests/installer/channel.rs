use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct Channel(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ChannelError {
    #[error("Channel must not be empty")]
    Empty,
    #[error(
        "Channel must not have more than {} characters but has {_0}",
        Channel::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Channel {
    pub const MAX_CHAR_LENGTH: usize = 16;

    /// Creates a new `Channel` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the channel is empty or more than 16 characters long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Channel;
    /// # use winget_types::installer::ChannelError;
    ///
    /// # fn main() -> Result<(), ChannelError>  {
    /// let channel = Channel::new("beta")?;
    ///
    /// assert_eq!(channel.as_str(), "beta");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(channel: T) -> Result<Self, ChannelError> {
        let channel_str = channel.as_ref();

        if channel_str.is_empty() {
            return Err(ChannelError::Empty);
        }

        let char_count = channel_str.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(ChannelError::TooLong(char_count));
        }

        Ok(Self(channel.into()))
    }

    /// Creates a new `Channel` from any type that implements `Into<CompactString>` without checking
    /// its validity.
    ///
    /// # Safety
    ///
    /// The channel must not be empty or more than 16 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(channel: T) -> Self {
        Self(channel.into())
    }

    /// Extracts a string slice containing the entire `Channel`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Channel {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Channel {
    type Err = ChannelError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for Channel {
    type Error = ChannelError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{Channel, ChannelError};

    #[rstest]
    #[case("stable")]
    #[case("beta")]
    #[case("dev")]
    #[case("nightly")]
    #[case("canary")]
    fn valid_channel(#[case] channel: &str) {
        assert!(channel.parse::<Channel>().is_ok());
    }

    #[test]
    fn empty_channel() {
        assert_eq!("".parse::<Channel>(), Err(ChannelError::Empty));
    }

    #[test]
    fn channel_too_long() {
        assert_eq!(
            "frequent_nightly_builds".parse::<Channel>(),
            Err(ChannelError::TooLong(23))
        );
    }
}

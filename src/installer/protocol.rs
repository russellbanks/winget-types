use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "CompactString"))]
#[repr(transparent)]
pub struct Protocol(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ProtocolError {
    #[error("Protocol must not be empty")]
    Empty,
    #[error(
        "Protocol must not have more than {} characters but has {_0}",
        Protocol::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Protocol {
    pub const MAX_CHAR_LENGTH: usize = 2048;

    /// Creates a new `Protocol` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the protocol is empty or more than 2048 characters long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Protocol;
    /// # use winget_types::installer::ProtocolError;
    ///
    /// # fn main() -> Result<(), ProtocolError>  {
    /// let command = Protocol::new("ftp")?;
    ///
    /// assert_eq!(command.as_str(), "ftp");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(protocol: T) -> Result<Self, ProtocolError> {
        let channel_str = protocol.as_ref();

        if channel_str.is_empty() {
            return Err(ProtocolError::Empty);
        }

        let char_count = channel_str.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(ProtocolError::TooLong(char_count));
        }

        Ok(Self(protocol.into()))
    }

    /// Creates a new `Protocol` from any type that implements `Into<CompactString>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The command must not be empty or more than 2048 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(protocol: T) -> Self {
        Self(protocol.into())
    }

    /// Extracts a string slice containing the entire `Protocol`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Protocol {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Protocol {
    type Err = ProtocolError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<CompactString> for Protocol {
    type Error = ProtocolError;

    #[inline]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn serialize_protocol() {
        use indoc::indoc;

        use super::Protocol;

        assert_eq!(
            serde_yaml::to_string(&Protocol::new("ftp").unwrap()).unwrap(),
            indoc! {"
                ftp
            "}
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_protocol() {
        use indoc::indoc;

        use super::Protocol;

        assert_eq!(
            serde_yaml::from_str::<Protocol>(&indoc! {"
                ftp
            "})
            .unwrap(),
            Protocol::new("ftp").unwrap(),
        );
    }
}

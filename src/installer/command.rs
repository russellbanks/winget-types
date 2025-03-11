use core::{fmt, str::FromStr};

use compact_str::CompactString;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct Command(CompactString);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CommandError {
    #[error("Command must not be empty")]
    Empty,
    #[error(
        "Command must not have more than {} characters but has {_0}",
        Command::MAX_CHAR_LENGTH
    )]
    TooLong(usize),
}

impl Command {
    pub const MAX_CHAR_LENGTH: usize = 40;

    /// Creates a new `Command` from any type that implements `AsRef<str>` and
    /// `Into<CompactString>`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the command is empty or more than 40 characters long.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::Command;
    /// # use winget_types::installer::CommandError;
    ///
    /// # fn main() -> Result<(), CommandError>  {
    /// let command = Command::new("pwsh")?;
    ///
    /// assert_eq!(command.as_str(), "pwsh");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<T: AsRef<str> + Into<CompactString>>(command: T) -> Result<Self, CommandError> {
        let command_str = command.as_ref();

        if command_str.is_empty() {
            return Err(CommandError::Empty);
        }

        let char_count = command_str.chars().count();
        if char_count > Self::MAX_CHAR_LENGTH {
            return Err(CommandError::TooLong(char_count));
        }

        Ok(Self(command.into()))
    }

    /// Creates a new `Command` from any type that implements `Into<CompactString>` without
    /// checking its validity.
    ///
    /// # Safety
    ///
    /// The command must not be empty or more than 40 characters long.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<CompactString>>(command: T) -> Self {
        Self(command.into())
    }

    /// Extracts a string slice containing the entire `Command`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Command {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Command {
    type Err = CommandError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for Command {
    type Error = CommandError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

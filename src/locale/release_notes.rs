use alloc::{borrow::Cow, string::String};
use core::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
#[repr(transparent)]
pub struct ReleaseNotes(String);

#[derive(Error, Debug, Eq, PartialEq)]
#[error("Release notes cannot be empty")]
pub struct ReleaseNotesError;

impl ReleaseNotes {
    pub const MAX_CHAR_LENGTH: usize = 10_000;

    /// Creates a new `ReleaseNotes` from any type that implements `AsRef<str>`.
    ///
    /// Release notes greater than 10,000 characters will be truncated to the first line where the
    /// total number of characters of that line and all previous lines are less than or equal to
    /// 10,000 characters.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the release notes are empty.
    pub fn new<T: AsRef<str>>(release_notes: T) -> Result<Self, ReleaseNotesError> {
        let result =
            truncate_with_lines::<{ Self::MAX_CHAR_LENGTH }>(release_notes.as_ref().trim());
        if result.is_empty() {
            Err(ReleaseNotesError)
        } else {
            Ok(Self(result.into_owned()))
        }
    }

    /// Creates a new `ReleaseNotes` from any type that implements `<Into<String>>` without checking
    /// whether it is empty.
    ///
    /// # Safety
    ///
    /// The value must not be empty.
    #[must_use]
    #[inline]
    pub unsafe fn new_unchecked<T: Into<String>>(release_notes: T) -> Self {
        Self(release_notes.into())
    }

    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for ReleaseNotes {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ReleaseNotes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ReleaseNotes {
    type Err = ReleaseNotesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for ReleaseNotes {
    type Error = ReleaseNotesError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

fn truncate_with_lines<const N: usize>(value: &str) -> Cow<str> {
    if value.chars().count() <= N {
        return Cow::Borrowed(value);
    }

    let mut result = String::new();
    let mut current_size = 0;

    for (index, line) in value.lines().enumerate() {
        let prospective_size = current_size + line.chars().count() + "\n".len();
        if prospective_size > N {
            break;
        }
        if index != 0 {
            result.push('\n');
        }
        result.push_str(line);
        current_size = prospective_size;
    }

    Cow::Owned(result)
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use super::truncate_with_lines;

    #[test]
    fn test_truncate_to_lines() {
        use core::fmt::Write;

        const CHAR_LIMIT: usize = 100;

        let mut buffer = String::new();
        let mut line_count = 0;
        while buffer.chars().count() <= CHAR_LIMIT {
            line_count += 1;
            writeln!(buffer, "Line {line_count}").unwrap();
        }
        let formatted = truncate_with_lines::<CHAR_LIMIT>(&buffer);
        let formatted_char_count = formatted.chars().count();
        assert!(formatted_char_count < buffer.chars().count());
        assert_eq!(formatted.trim().chars().count(), formatted_char_count);
    }
}

use core::{fmt, str::FromStr};

use thiserror::Error;

use crate::utils::RelativeDir;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Scope {
    User,
    Machine,
}

const USER: &str = "user";
const MACHINE: &str = "machine";

impl Scope {
    #[must_use]
    pub fn find_in<T: AsRef<[u8]>>(value: T) -> Option<Self> {
        let bytes = value.as_ref();

        let value_contains = |scope: Self| -> Option<Self> {
            bytes
                .windows(scope.as_str().len())
                .any(|window| window.eq_ignore_ascii_case(scope.as_str().as_bytes()))
                .then_some(scope)
        };

        value_contains(Self::User).or_else(|| value_contains(Self::Machine))
    }

    #[must_use]
    pub fn from_install_directory<T: AsRef<str>>(install_directory: T) -> Option<Self> {
        const USER_INSTALL_DIRS: [&str; 2] = [RelativeDir::APP_DATA, RelativeDir::LOCAL_APP_DATA];
        const MACHINE_INSTALL_DIRS: [&str; 7] = [
            RelativeDir::PROGRAM_FILES_64,
            RelativeDir::PROGRAM_FILES_32,
            RelativeDir::COMMON_FILES_64,
            RelativeDir::COMMON_FILES_32,
            RelativeDir::PROGRAM_DATA,
            RelativeDir::WINDOWS_DIR,
            RelativeDir::SYSTEM_ROOT,
        ];

        let install_directory = install_directory.as_ref();

        USER_INSTALL_DIRS
            .iter()
            .any(|directory| install_directory.starts_with(directory))
            .then_some(Self::User)
            .or_else(|| {
                MACHINE_INSTALL_DIRS
                    .iter()
                    .any(|directory| install_directory.starts_with(directory))
                    .then_some(Self::Machine)
            })
    }

    /// Returns `true` if the scope is user.
    #[must_use]
    #[inline]
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::User)
    }

    /// Returns `true` if the scope is machine.
    #[must_use]
    #[inline]
    pub const fn is_machine(&self) -> bool {
        matches!(self, Self::Machine)
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::User => USER,
            Self::Machine => MACHINE,
        }
    }
}

impl AsRef<str> for Scope {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
#[error("Scope did not match either `{USER}` or `{MACHINE}`")]
pub struct ScopeParseError;

impl FromStr for Scope {
    type Err = ScopeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            USER => Ok(Self::User),
            MACHINE => Ok(Self::Machine),
            _ => Err(ScopeParseError),
        }
    }
}

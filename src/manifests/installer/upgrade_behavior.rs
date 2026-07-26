use core::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum UpgradeBehavior {
    Install,
    UninstallPrevious,
    Deny,
}

const INSTALL: &str = "Install";
const UNINSTALL_PREVIOUS: &str = "UninstallPrevious";
const DENY: &str = "Deny";

impl fmt::Display for UpgradeBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Install => f.write_str(INSTALL),
            Self::UninstallPrevious => f.write_str(UNINSTALL_PREVIOUS),
            Self::Deny => f.write_str(DENY),
        }
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
#[error("Upgrade behavior did not match any of `{INSTALL}`, `{UNINSTALL_PREVIOUS}`, or `{DENY}`")]
pub struct UpgradeBehaviorParseError;

impl FromStr for UpgradeBehavior {
    type Err = UpgradeBehaviorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            INSTALL => Ok(Self::Install),
            UNINSTALL_PREVIOUS => Ok(Self::UninstallPrevious),
            DENY => Ok(Self::Deny),
            _ => Err(UpgradeBehaviorParseError),
        }
    }
}

use core::{fmt, str::FromStr};

use thiserror::Error;

use super::nested::installer_type::NestedInstallerType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
pub enum InstallerType {
    Msix,
    Msi,
    Appx,
    Exe,
    Zip,
    Inno,
    Nullsoft,
    Wix,
    Burn,
    Pwa,
    Portable,
    Font,
}

impl InstallerType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Msix => "msix",
            Self::Msi => "msi",
            Self::Appx => "appx",
            Self::Exe => "exe",
            Self::Zip => "zip",
            Self::Inno => "inno",
            Self::Nullsoft => "nullsoft",
            Self::Wix => "wix",
            Self::Burn => "burn",
            Self::Pwa => "pwa",
            Self::Portable => "portable",
            Self::Font => "font",
        }
    }
}

impl AsRef<str> for InstallerType {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<InstallerType> for NestedInstallerType {
    type Error = ();

    fn try_from(value: InstallerType) -> Result<Self, Self::Error> {
        match value {
            InstallerType::Msix => Ok(Self::Msix),
            InstallerType::Msi => Ok(Self::Msi),
            InstallerType::Appx => Ok(Self::Appx),
            InstallerType::Exe => Ok(Self::Exe),
            InstallerType::Inno => Ok(Self::Inno),
            InstallerType::Nullsoft => Ok(Self::Nullsoft),
            InstallerType::Wix => Ok(Self::Wix),
            InstallerType::Burn => Ok(Self::Burn),
            InstallerType::Portable => Ok(Self::Portable),
            InstallerType::Font => Ok(Self::Font),
            InstallerType::Zip | InstallerType::Pwa => Err(()),
        }
    }
}

impl fmt::Display for InstallerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
#[error("Installer type did not match a valid lowercase installer type")]
pub struct InstallerTypeParseError;

impl FromStr for InstallerType {
    type Err = InstallerTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "msix" => Ok(Self::Msix),
            "msi" => Ok(Self::Msi),
            "appx" => Ok(Self::Appx),
            "exe" => Ok(Self::Exe),
            "zip" => Ok(Self::Zip),
            "inno" => Ok(Self::Inno),
            "nullsoft" => Ok(Self::Nullsoft),
            "wix" => Ok(Self::Wix),
            "burn" => Ok(Self::Burn),
            "pwa" => Ok(Self::Pwa),
            "portable" => Ok(Self::Portable),
            "font" => Ok(Self::Font),
            _ => Err(InstallerTypeParseError),
        }
    }
}

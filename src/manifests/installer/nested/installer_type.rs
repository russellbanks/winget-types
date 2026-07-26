use core::fmt;

use crate::installer::InstallerType;

/// Enumeration of supported nested installer shared contained inside an archive file
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
pub enum NestedInstallerType {
    Msix,
    Msi,
    Appx,
    Exe,
    Inno,
    Nullsoft,
    Wix,
    Burn,
    Portable,
    Font,
}

impl NestedInstallerType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Msix => "msix",
            Self::Msi => "msi",
            Self::Appx => "appx",
            Self::Exe => "exe",
            Self::Inno => "inno",
            Self::Nullsoft => "nullsoft",
            Self::Wix => "wix",
            Self::Burn => "burn",
            Self::Portable => "portable",
            Self::Font => "font",
        }
    }
}

impl From<NestedInstallerType> for InstallerType {
    fn from(value: NestedInstallerType) -> Self {
        match value {
            NestedInstallerType::Msix => Self::Msix,
            NestedInstallerType::Msi => Self::Msi,
            NestedInstallerType::Appx => Self::Appx,
            NestedInstallerType::Exe => Self::Exe,
            NestedInstallerType::Inno => Self::Inno,
            NestedInstallerType::Nullsoft => Self::Nullsoft,
            NestedInstallerType::Wix => Self::Wix,
            NestedInstallerType::Burn => Self::Burn,
            NestedInstallerType::Portable => Self::Portable,
            NestedInstallerType::Font => Self::Font,
        }
    }
}

impl fmt::Display for NestedInstallerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum RepairBehavior {
    Modify,
    Uninstaller,
    Installer,
}

impl fmt::Display for RepairBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Modify => f.write_str("Modify"),
            Self::Uninstaller => f.write_str("Uninstaller"),
            Self::Installer => f.write_str("Installer"),
        }
    }
}

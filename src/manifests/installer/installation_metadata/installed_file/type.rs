use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum InstalledFileType {
    Launch,
    Uninstall,
    Other,
}

impl fmt::Display for InstalledFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Launch => write!(f, "launch"),
            Self::Uninstall => write!(f, "uninstall"),
            Self::Other => write!(f, "other"),
        }
    }
}

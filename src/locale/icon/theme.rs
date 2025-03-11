use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum IconTheme {
    Default,
    Light,
    Dark,
    HighContrast,
}

impl fmt::Display for IconTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => f.write_str("Default"),
            Self::Light => f.write_str("Light"),
            Self::Dark => f.write_str("Dark"),
            Self::HighContrast => f.write_str("High contrast"),
        }
    }
}

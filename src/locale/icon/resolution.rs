use core::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum IconResolution {
    Custom,
    #[cfg_attr(feature = "serde", serde(rename = "16x16"))]
    Size16,
    #[cfg_attr(feature = "serde", serde(rename = "20x20"))]
    Size20,
    #[cfg_attr(feature = "serde", serde(rename = "24x24"))]
    Size24,
    #[cfg_attr(feature = "serde", serde(rename = "30x30"))]
    Size30,
    #[cfg_attr(feature = "serde", serde(rename = "32x32"))]
    Size32,
    #[cfg_attr(feature = "serde", serde(rename = "36x36"))]
    Size36,
    #[cfg_attr(feature = "serde", serde(rename = "40x40"))]
    Size40,
    #[cfg_attr(feature = "serde", serde(rename = "48x48"))]
    Size48,
    #[cfg_attr(feature = "serde", serde(rename = "60x60"))]
    Size60,
    #[cfg_attr(feature = "serde", serde(rename = "64x64"))]
    Size64,
    #[cfg_attr(feature = "serde", serde(rename = "72x72"))]
    Size72,
    #[cfg_attr(feature = "serde", serde(rename = "80x80"))]
    Size80,
    #[cfg_attr(feature = "serde", serde(rename = "96x96"))]
    Size96,
    #[cfg_attr(feature = "serde", serde(rename = "256x256"))]
    Size256,
}

impl fmt::Display for IconResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => f.write_str("custom"),
            Self::Size16 => f.write_str("16x16"),
            Self::Size20 => f.write_str("20x20"),
            Self::Size24 => f.write_str("24x24"),
            Self::Size30 => f.write_str("30x30"),
            Self::Size32 => f.write_str("32x32"),
            Self::Size36 => f.write_str("36x36"),
            Self::Size40 => f.write_str("40x40"),
            Self::Size48 => f.write_str("48x48"),
            Self::Size60 => f.write_str("60x60"),
            Self::Size64 => f.write_str("64x64"),
            Self::Size72 => f.write_str("72x72"),
            Self::Size80 => f.write_str("80x80"),
            Self::Size96 => f.write_str("96x96"),
            Self::Size256 => f.write_str("256x256"),
        }
    }
}

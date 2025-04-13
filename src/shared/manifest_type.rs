use core::fmt;

use icu_locid::LanguageIdentifier;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ManifestType {
    #[default]
    Installer,
    DefaultLocale,
    Locale,
    Version,
}

#[cfg(feature = "serde")]
impl ManifestType {
    pub(crate) const fn installer() -> Self {
        Self::Installer
    }

    pub(crate) const fn default_locale() -> Self {
        Self::DefaultLocale
    }

    pub(crate) const fn locale() -> Self {
        Self::Locale
    }

    pub(crate) const fn version() -> Self {
        Self::Version
    }
}

impl fmt::Display for ManifestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Installer => f.write_str("Installer"),
            Self::DefaultLocale => f.write_str("DefaultLocale"),
            Self::Locale => f.write_str("Locale"),
            Self::Version => f.write_str("Version"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ManifestTypeWithLocale {
    Installer,
    Locale(LanguageIdentifier),
    Version,
}

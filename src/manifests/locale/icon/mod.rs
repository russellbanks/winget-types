mod file_type;
mod resolution;
mod theme;

pub use file_type::IconFileType;
pub use resolution::IconResolution;
pub use theme::IconTheme;
use url::Url;

use crate::Sha256String;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Icon {
    /// The url of the hosted icon file
    #[cfg_attr(feature = "serde", serde(rename = "IconUrl"))]
    pub url: Url,
    /// The icon file type
    #[cfg_attr(feature = "serde", serde(rename = "IconFileType"))]
    pub file_type: IconFileType,
    /// Optional icon resolution
    #[cfg_attr(
        feature = "serde",
        serde(rename = "IconResolution", skip_serializing_if = "Option::is_none")
    )]
    pub resolution: Option<IconResolution>,
    /// Optional icon theme
    #[cfg_attr(
        feature = "serde",
        serde(rename = "IconTheme", skip_serializing_if = "Option::is_none")
    )]
    pub theme: Option<IconTheme>,
    /// Optional Sha256 of the icon file
    #[cfg_attr(
        feature = "serde",
        serde(rename = "IconSha256", skip_serializing_if = "Option::is_none")
    )]
    pub sha256: Option<Sha256String>,
}

impl Icon {
    /// Creates a new [`Icon`] from a [`Url`] and an [`IconFileType`].
    pub fn new(url: Url, file_type: IconFileType) -> Self {
        Self {
            url,
            file_type,
            resolution: None,
            theme: None,
            sha256: None,
        }
    }
}

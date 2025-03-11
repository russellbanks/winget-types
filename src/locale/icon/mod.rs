pub mod file_type;
pub mod resolution;
pub mod theme;

use url::Url;

use crate::{
    locale::icon::{file_type::IconFileType, resolution::IconResolution, theme::IconTheme},
    shared::Sha256String,
};

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
    pub sha_256: Option<Sha256String>,
}

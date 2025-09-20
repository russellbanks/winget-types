pub use authentication_type::{AuthenticationType, AuthenticationTypeParseError};
pub use info::{MicrosoftEntraIdAuthenticationInfo, Resource, ResourceError};

mod authentication_type;
pub mod info;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Authentication {
    /// This field controls whether to use no authentication, Entra ID, or Entra ID for Azure
    /// Blob Storage.
    #[cfg_attr(feature = "serde", serde(rename = "AuthenticationType"))]
    pub r#type: AuthenticationType,

    /// This field controls the authentication details used when downloading or installing packages
    /// from Entra Id secured private sources.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "MicrosoftEntraIdAuthenticationInfo::is_empty")
    )]
    pub microsoft_entra_id_authentication_info: MicrosoftEntraIdAuthenticationInfo,
}

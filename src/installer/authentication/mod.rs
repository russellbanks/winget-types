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
    r#type: AuthenticationType,

    /// This field controls the authentication details used when downloading or installing packages
    /// from Entra Id secured private sources.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "MicrosoftEntraIdAuthenticationInfo::is_empty")
    )]
    microsoft_entra_id_authentication_info: MicrosoftEntraIdAuthenticationInfo,
}

impl Authentication {
    /// Returns the authentication type which controls whether to use no authentication, Entra ID,
    /// or Entra ID for Azure Blob Storage.
    #[must_use]
    #[inline]
    pub const fn r#type(&self) -> AuthenticationType {
        self.r#type
    }

    /// Returns the Microsoft Entra ID authentication info which controls the authentication details
    /// used when downloading or installing packages from Entra Id secured private sources.
    #[must_use]
    #[inline]
    pub const fn microsoft_entra_id_authentication_info(
        &self,
    ) -> &MicrosoftEntraIdAuthenticationInfo {
        &self.microsoft_entra_id_authentication_info
    }
}

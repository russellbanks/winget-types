use core::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum AuthenticationType {
    #[default]
    None,
    MicrosoftEntraId,
    MicrosoftEntraIdForAzureBlobStorage,
}

const NONE: &str = "none";
const MICROSOFT_ENTRA_ID: &str = "microsoftEntraId";
const MICROSOFT_ENTRA_ID_FOR_AZURE_BLOB_STORAGE: &str = "microsoftEntraIdForAzureBlobStorage";

impl AuthenticationType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => NONE,
            Self::MicrosoftEntraId => MICROSOFT_ENTRA_ID,
            Self::MicrosoftEntraIdForAzureBlobStorage => MICROSOFT_ENTRA_ID_FOR_AZURE_BLOB_STORAGE,
        }
    }
}

impl fmt::Display for AuthenticationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
#[error(
    "Upgrade behavior did not match any of `{NONE}`, `{MICROSOFT_ENTRA_ID}`, or `{MICROSOFT_ENTRA_ID_FOR_AZURE_BLOB_STORAGE}`"
)]
pub struct AuthenticationTypeParseError;

impl FromStr for AuthenticationType {
    type Err = AuthenticationTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            NONE => Ok(Self::None),
            MICROSOFT_ENTRA_ID => Ok(Self::MicrosoftEntraId),
            MICROSOFT_ENTRA_ID_FOR_AZURE_BLOB_STORAGE => {
                Ok(Self::MicrosoftEntraIdForAzureBlobStorage)
            }
            _ => Err(AuthenticationTypeParseError),
        }
    }
}

use crate::{
    installer::{installer_return_code::InstallerReturnCode, return_response::ReturnResponse},
    shared::url::DecodedUrl,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ExpectedReturnCodes {
    /// This key represents any status code returned by the installer representing a condition other
    /// than zero. MSIX and MSI packages have well known return codes. This is primarily intended
    /// for executable installers that have custom or unique return codes that can be mapped to a
    /// return response.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub installer_return_code: Option<InstallerReturnCode>,

    /// This key represents a return response to display when an installer returns an expected
    /// return code. MSIX and MSI packages have well known return codes. This is primarily intended
    /// for executable installers that have custom or unique return codes that can be mapped to a
    /// return response.
    pub return_response: ReturnResponse,

    /// This key represents a return response URL to display when an installer returns an expected
    /// return code. MSIX and MSI packages have well known return codes. This is primarily intended
    /// for executable installers that have custom or unique return codes that can be mapped to a
    /// return response.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub return_response_url: Option<DecodedUrl>,
}

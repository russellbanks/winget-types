use crate::{
    installer::{installer_return_code::InstallerReturnCode, return_response::ReturnResponse},
    url::DecodedUrl,
};

/// An expected return code.
///
/// MSIX and MSI packages have well known return codes so this is primarily
/// intended for executable installers that have custom or unique
/// return codes that can be mapped to a return response.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ExpectedReturnCode {
    /// Any status code returned by the installer representing a condition other
    /// than zero.
    installer_return_code: InstallerReturnCode,

    /// A return response to display when an installer returns an expected
    /// return code.
    return_response: ReturnResponse,

    /// A return response URL to display when an installer returns an expected
    /// return code.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    return_response_url: Option<DecodedUrl>,
}

impl ExpectedReturnCode {
    /// Creates a new [`ExpectedReturnCode`] from an [`InstallerReturnCode`] and
    /// [`ReturnResponse].
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::{ExpectedReturnCode, InstallerReturnCode, ReturnResponse};
    ///
    ///# fn test() -> Option<()> {
    /// let expected_return_code = ExpectedReturnCode::new(
    ///     InstallerReturnCode::new(3010)?,
    ///     ReturnResponse::RebootRequiredToFinish,
    /// );
    ///
    /// assert_eq!(expected_return_code.installer_return_code().get(), 3010);
    /// assert_eq!(expected_return_code.return_response(), ReturnResponse::RebootRequiredToFinish);
    /// # Some(())
    /// # }
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(
        installer_return_code: InstallerReturnCode,
        return_response: ReturnResponse,
    ) -> Self {
        Self {
            installer_return_code,
            return_response,
            return_response_url: None,
        }
    }

    /// Creates a new [`ExpectedReturnCode`] from an [`InstallerReturnCode`], a
    /// [`ReturnResponse`], and a [`DecodedUrl`].
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::{ExpectedReturnCode, InstallerReturnCode, ReturnResponse};
    /// use winget_types::url::DecodedUrl;
    ///
    ///# fn main() -> Result<(), url::ParseError> {
    /// let expected_return_code = ExpectedReturnCode::new_with_response_url(
    ///     InstallerReturnCode::new(-1).unwrap(),
    ///     ReturnResponse::CancelledByUser,
    ///     "https://example.com/".parse()?,
    /// );
    ///
    /// assert_eq!(expected_return_code.installer_return_code().get(), -1);
    /// assert_eq!(expected_return_code.return_response(), ReturnResponse::CancelledByUser);
    /// assert_eq!(
    ///     expected_return_code.return_response_url().map(DecodedUrl::as_str),
    ///     Some("https://example.com/")
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    #[inline]
    pub const fn new_with_response_url(
        installer_return_code: InstallerReturnCode,
        return_response: ReturnResponse,
        return_response_url: DecodedUrl,
    ) -> Self {
        Self {
            installer_return_code,
            return_response,
            return_response_url: Some(return_response_url),
        }
    }

    /// Returns the [`InstallerReturnCode`].
    #[must_use]
    #[inline]
    pub const fn installer_return_code(&self) -> InstallerReturnCode {
        self.installer_return_code
    }

    /// Returns the [`ReturnResponse`].
    #[must_use]
    #[inline]
    pub const fn return_response(&self) -> ReturnResponse {
        self.return_response
    }

    /// Returns the return response URL or `None` if one is not defined.
    #[must_use]
    #[inline]
    pub const fn return_response_url(&self) -> Option<&DecodedUrl> {
        self.return_response_url.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::ExpectedReturnCode;

    #[test]
    fn size() {
        assert_eq!(size_of::<ExpectedReturnCode>(), 104);
    }
}

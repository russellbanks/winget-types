use alloc::string::String;

use url::Url;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Agreement {
    /// The label for a package agreement.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AgreementLabel", skip_serializing_if = "Option::is_none")
    )]
    pub label: Option<String>,

    /// The text or body of a package agreement.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "Agreement", skip_serializing_if = "Option::is_none")
    )]
    pub text: Option<String>,

    /// The URL for a package agreement.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "AgreementUrl", skip_serializing_if = "Option::is_none")
    )]
    pub url: Option<Url>,
}

impl Agreement {
    /// Returns `true` if all the `Agreement` fields are `None`.
    ///
    /// # Examples
    /// ```
    /// use winget_types::locale::Agreement;
    ///
    /// let mut agreement = Agreement::default();
    /// assert!(agreement.is_empty());
    ///
    /// agreement.label = Some("An agreement".into());
    /// assert!(!agreement.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.label.is_none() && self.text.is_none() && self.url.is_none()
    }
}

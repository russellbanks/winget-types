mod label;

pub use label::DocumentLabel;
use url::Url;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Documentation {
    /// The label of the documentation for providing software guides such as manuals and
    /// troubleshooting URLs.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub document_label: Option<DocumentLabel>,

    /// The URL for a documentation.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub document_url: Option<Url>,
}

impl Documentation {
    /// Returns `true` if all fields of the `Documentation` are empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::locale::{Documentation, DocumentLabel};
    ///
    /// let mut documentation = Documentation::default();
    /// assert!(documentation.is_empty());
    ///
    /// documentation.document_label = Some(DocumentLabel::new("Label").unwrap());
    /// assert!(!documentation.is_empty());
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.document_label.is_none() && self.document_url.is_none()
    }
}

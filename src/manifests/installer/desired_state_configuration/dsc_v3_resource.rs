use alloc::string::String;

/// A DSC v3 resource, named as `Publisher.Product.Component/ResourceName`.
///
/// The product and component portions of the name are optional.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct DscV3Resource {
    r#type: String,
}

impl DscV3Resource {
    /// Creates a new [DscV3Resource] from a type.
    #[must_use]
    pub fn new<T: Into<String>>(r#type: T) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }

    /// Returns the type.
    #[must_use]
    #[inline]
    pub const fn r#type(&self) -> &str {
        self.r#type.as_str()
    }
}

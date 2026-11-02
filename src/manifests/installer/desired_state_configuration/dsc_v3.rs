use alloc::vec::Vec;

use super::DscV3Resource;

/// DSC v3 resources contained in a package.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct DscV3 {
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub resources: Vec<DscV3Resource>,
}

impl DscV3 {
    /// Returns `true` if the resources are empty.
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns the resources as a slice.
    #[must_use]
    #[inline]
    pub const fn resources(&self) -> &[DscV3Resource] {
        self.resources.as_slice()
    }
}

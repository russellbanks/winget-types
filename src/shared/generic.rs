use super::ManifestType;

/// A manifest where the only field is the type of the manifest itself. Useful for deserializing
/// once into this type to determine which manifest to properly deserialize into.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct GenericManifest {
    #[cfg_attr(feature = "serde", serde(rename = "ManifestType"))]
    pub r#type: ManifestType,
}

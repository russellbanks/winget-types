use super::portable_command_alias::PortableCommandAlias;
use crate::Path;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct NestedInstallerFiles {
    pub relative_file_path: Path,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub portable_command_alias: Option<PortableCommandAlias>,
}

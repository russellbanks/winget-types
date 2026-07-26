mod r#type;

use alloc::string::String;

use bon::Builder;
use r#type::InstalledFileType;

use crate::{Path, PathBuf, Sha256String};

#[derive(Builder, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct InstalledFile {
    /// The path to the installed file relative to the default install location.
    #[cfg_attr(feature = "serde", serde(rename = "RelativeFilePath"))]
    file_path: PathBuf,

    /// The Sha256 hash of the installed file.
    #[cfg_attr(
        feature = "serde",
        serde(rename = "FileSha256", skip_serializing_if = "Option::is_none")
    )]
    sha256: Option<Sha256String>,

    /// The type of the installed file - [`launch`], [`uninstall`], or [`other`].
    /// If not specified, the file is treated as [`other`].
    ///
    /// [`launch`]: InstalledFileType::Launch
    /// [`uninstall`]: InstalledFileType::Uninstall
    /// [`other`]: InstalledFileType::Other
    #[cfg_attr(
        feature = "serde",
        serde(rename = "FileType", skip_serializing_if = "Option::is_none")
    )]
    file_type: Option<InstalledFileType>,

    /// The parameter to use for invocable files.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    invocation_parameter: Option<String>,

    /// The display name to use for invocable files.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    display_name: Option<String>,
}

impl InstalledFile {
    /// Returns the file's relative path.
    #[must_use]
    pub fn relative_file_path(&self) -> &Path {
        cfg_select! {
            feature = "std" => self.file_path.as_path(),
            _ => self.file_path.as_str()
        }
    }

    /// Returns the file's SHA-256 hash if present.
    #[must_use]
    pub const fn sha256(&self) -> Option<&Sha256String> {
        self.sha256.as_ref()
    }

    /// Returns the file's [`InstalledFileType`] if present.
    #[must_use]
    pub const fn file_type(&self) -> Option<InstalledFileType> {
        self.file_type
    }

    /// Returns the invocation parameter if present.
    #[must_use]
    pub fn invocation_parameter(&self) -> Option<&str> {
        self.invocation_parameter.as_deref()
    }

    /// Returns the display name if present.
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::{InstalledFile, r#type::InstalledFileType};

    #[test]
    fn builder() {
        let installed_file = InstalledFile::builder()
            .file_path(r"path\\to\\file".into())
            .file_type(InstalledFileType::Launch)
            .build();

        assert_eq!(installed_file.relative_file_path(), r"path\\to\\file");
        assert!(installed_file.sha256().is_none());
        assert_eq!(installed_file.file_type(), Some(InstalledFileType::Launch));
        assert!(installed_file.invocation_parameter().is_none());
        assert!(installed_file.display_name().is_none());
    }
}

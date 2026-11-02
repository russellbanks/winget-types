use alloc::{string::String, vec::Vec};

use super::PowerShellResource;
use crate::url::DecodedUrl;

/// A PowerShell module containing DSC resources.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct PowerShellModule {
    pub repository_url: DecodedUrl,
    pub module_name: String,
    pub resources: Vec<PowerShellResource>,
}

impl PowerShellModule {
    /// Creates a new [PowerShellModule] from a repository URL, a module name,
    /// and an iterator of [PowerShellResource].
    #[must_use]
    pub fn new<T: Into<String>, I: IntoIterator<Item = PowerShellResource>>(
        repository_url: DecodedUrl,
        module_name: T,
        resources: I,
    ) -> Self {
        Self {
            repository_url,
            module_name: module_name.into(),
            resources: resources.into_iter().collect(),
        }
    }

    /// Returns the repository URL.
    #[must_use]
    #[inline]
    pub const fn repository_url(&self) -> &DecodedUrl {
        &self.repository_url
    }

    /// Returns the module name as a string slice.
    #[must_use]
    #[inline]
    pub const fn module_name(&self) -> &str {
        self.module_name.as_str()
    }

    /// Returns the resources as a slice.
    #[must_use]
    #[inline]
    pub const fn resources(&self) -> &[PowerShellResource] {
        self.resources.as_slice()
    }
}

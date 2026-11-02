use alloc::string::String;

/// A DSC resource in a PowerShell module.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct PowerShellResource {
    pub name: String,
}

impl PowerShellResource {
    /// Creates a new [PowerShellResource] from a name.
    #[must_use]
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self { name: name.into() }
    }

    /// Returns the name.
    #[must_use]
    #[inline]
    pub const fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the name as a string slice.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.name()
    }
}

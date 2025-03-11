use alloc::string::String;

use compact_str::CompactString;

use crate::{Version, installer::installer_type::InstallerType, locale::DefaultLocaleManifest};

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct AppsAndFeaturesEntry {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub display_name: Option<CompactString>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub publisher: Option<CompactString>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub display_version: Option<Version>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub product_code: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub upgrade_code: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub installer_type: Option<InstallerType>,
}

impl AppsAndFeaturesEntry {
    /// Creates a new, empty`AppsAndFeaturesEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::AppsAndFeaturesEntry;
    ///
    /// let mut entry = AppsAndFeaturesEntry::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            display_name: None,
            publisher: None,
            display_version: None,
            product_code: None,
            upgrade_code: None,
            installer_type: None,
        }
    }

    #[must_use]
    pub fn with_display_name<T, U>(mut self, display_name: T) -> Self
    where
        T: Into<Option<U>>,
        U: Into<CompactString>,
    {
        self.display_name = display_name.into().map(U::into);
        self
    }

    #[must_use]
    pub fn with_publisher<T, U>(mut self, publisher: T) -> Self
    where
        T: Into<Option<U>>,
        U: Into<CompactString>,
    {
        self.publisher = publisher.into().map(U::into);
        self
    }

    #[must_use]
    pub fn with_display_version<T, U>(mut self, display_version: T) -> Self
    where
        T: Into<Option<U>>,
        U: Into<Version>,
    {
        self.display_version = display_version.into().map(U::into);
        self
    }

    #[must_use]
    pub fn with_product_code<T, U>(mut self, product_code: T) -> Self
    where
        T: Into<Option<U>>,
        U: Into<String>,
    {
        self.product_code = product_code.into().map(U::into);
        self
    }

    #[must_use]
    pub fn with_upgrade_code<T, U>(mut self, upgrade_code: T) -> Self
    where
        T: Into<Option<U>>,
        U: Into<String>,
    {
        self.upgrade_code = upgrade_code.into().map(U::into);
        self
    }

    #[must_use]
    pub fn with_installer_type<T, U>(mut self, installer_type: T) -> Self
    where
        T: Into<Option<U>>,
        U: Into<InstallerType>,
    {
        self.installer_type = installer_type.into().map(U::into);
        self
    }

    /// Returns `true` if all the `AppsAndFeatureEntry` fields are `None`.
    ///
    /// # Examples
    /// ```
    /// use winget_types::installer::{AppsAndFeaturesEntry, InstallerType};
    ///
    /// let mut arp_entry = AppsAndFeaturesEntry::default();
    /// assert!(arp_entry.is_empty());
    ///
    /// arp_entry.installer_type = Some(InstallerType::Burn);
    /// assert!(!arp_entry.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.display_name.is_none()
            && self.publisher.is_none()
            && self.display_version.is_none()
            && self.product_code.is_none()
            && self.upgrade_code.is_none()
            && self.installer_type.is_none()
    }

    /// Removes values that are equivalent to their respective value in the default locale manifest.
    ///
    /// `AppsAndFeaturesEntry` field -> default locale field:
    /// - Display name -> Package name
    /// - Publisher -> Publisher
    /// - Display version -> Package version
    pub fn deduplicate(&mut self, default_locale_manifest: &DefaultLocaleManifest) {
        if self.display_name.as_deref() == Some(default_locale_manifest.package_name.as_str()) {
            self.display_name = None;
        }

        if self.publisher.as_deref() == Some(default_locale_manifest.publisher.as_str()) {
            self.publisher = None;
        }

        if self
            .display_version
            .as_ref()
            .is_some_and(|display_version| {
                display_version == &default_locale_manifest.package_version
            })
        {
            self.display_version = None;
        }
    }
}

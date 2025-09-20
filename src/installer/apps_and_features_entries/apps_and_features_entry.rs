use alloc::string::String;
use core::fmt;

use bon::Builder;
use compact_str::CompactString;

use crate::{Version, installer::InstallerType, locale::DefaultLocaleManifest};

#[derive(Builder, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct AppsAndFeaturesEntry {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(into)]
    display_name: Option<CompactString>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(into)]
    publisher: Option<CompactString>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(into)]
    display_version: Option<Version>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(into)]
    product_code: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(into)]
    upgrade_code: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(into)]
    installer_type: Option<InstallerType>,
}

impl AppsAndFeaturesEntry {
    /// Returns the display name, if any.
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Returns the publisher, if any.
    #[must_use]
    pub fn publisher(&self) -> Option<&str> {
        self.publisher.as_deref()
    }

    /// Returns the display version, if any.
    #[must_use]
    pub const fn display_version(&self) -> Option<&Version> {
        self.display_version.as_ref()
    }

    /// Returns the product code, if any.
    #[must_use]
    pub fn product_code(&self) -> Option<&str> {
        self.product_code.as_deref()
    }

    /// Returns the upgrade code, if any.
    #[must_use]
    pub fn upgrade_code(&self) -> Option<&str> {
        self.upgrade_code.as_deref()
    }

    /// Returns the installer type, if any.
    #[must_use]
    pub const fn installer_type(&self) -> Option<InstallerType> {
        self.installer_type
    }

    /// Returns `true` if all the `AppsAndFeatureEntry` fields are `None`.
    ///
    /// # Examples
    /// ```
    /// # use winget_types::installer::{AppsAndFeaturesEntry, InstallerType};
    /// assert!(AppsAndFeaturesEntry::default().is_empty());
    ///
    /// let arp_entry = AppsAndFeaturesEntry::builder().installer_type(InstallerType::Burn).build();
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

impl fmt::Debug for AppsAndFeaturesEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppsAndFeaturesEntry")
            .field("DisplayName", &self.display_name())
            .field("Publisher", &self.publisher())
            .field("DisplayVersion", &self.display_version())
            .field("ProductCode", &self.product_code())
            .field("UpgradeCode", &self.upgrade_code())
            .field("InstallerType", &self.installer_type())
            .finish()
    }
}

impl Default for AppsAndFeaturesEntry {
    fn default() -> Self {
        Self::builder().build()
    }
}

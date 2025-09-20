mod custom;
mod install_location;
mod interactive;
mod log;
mod repair;
mod silent;
mod silent_with_progress;
mod switch;
mod upgrade;

use bon::Builder;

pub use super::switches::{
    custom::CustomSwitch, install_location::InstallLocationSwitch, interactive::InteractiveSwitch,
    log::LogSwitch, repair::RepairSwitch, silent::SilentSwitch,
    silent_with_progress::SilentWithProgressSwitch, upgrade::UpgradeSwitch,
};

#[derive(Builder, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct InstallerSwitches {
    /// Switches passed to the installer to provide a silent install experience.
    ///
    /// These would be used when the command `winget install <package> --silent` is executed.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) silent: Option<SilentSwitch>,

    /// Switches passed to the installer to provide a silent with progress install experience.
    ///
    /// This is intended to allow a progress indication to the user, and the indication may come
    /// from an installer UI dialogue, but it must not require user interaction to complete. The
    /// Windows Package Manager currently defaults to this install experience.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) silent_with_progress: Option<SilentWithProgressSwitch>,

    /// Switches passed to the installer to provide an interactive install experience.
    ///
    /// This is intended to allow a user to interact with the installer. These would be used when
    /// the command `winget install <package> --interactive` is executed.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) interactive: Option<InteractiveSwitch>,

    /// The path to install the package if the installer supports installing the package in a user
    /// configurable location.
    ///
    /// The `<INSTALLPATH>` token can be included in the switch value so the Windows Package Manager
    /// will replace the token with user provided path.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) install_location: Option<InstallLocationSwitch>,

    /// The path logs will be directed to if the installer supports specifying the log path in a
    /// user configurable location.
    ///
    /// The `<LOGPATH>` token can be included in the switch value so the Windows Package Manager
    /// will replace the token with user provided path.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) log: Option<LogSwitch>,

    /// The switches to be passed to the installer during an upgrade. This will happen only if the
    /// upgrade behavior is "install".
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) upgrade: Option<UpgradeSwitch>,

    /// Any switches the Windows Package Manager will pass to the installer in addition to `Silent`,
    /// `SilentWithProgress`, and `Interactive`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) custom: Option<CustomSwitch>,

    /// The switches to be passed during the repair of an existing installation.
    ///
    /// This will be passed to the installer, `ModifyPath` ARP command, or Uninstaller ARP command
    /// depending on the `RepairBehavior` specified in the manifest.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) repair: Option<RepairSwitch>,
}

impl InstallerSwitches {
    /// Returns the silent switch, if any.
    #[must_use]
    #[inline]
    pub const fn silent(&self) -> Option<&SilentSwitch> {
        self.silent.as_ref()
    }

    /// Returns the silent with progress switch, if any.
    #[must_use]
    #[inline]
    pub const fn silent_with_progress(&self) -> Option<&SilentWithProgressSwitch> {
        self.silent_with_progress.as_ref()
    }

    /// Returns the interactive switch, if any.
    #[must_use]
    #[inline]
    pub const fn interactive(&self) -> Option<&InteractiveSwitch> {
        self.interactive.as_ref()
    }

    /// Returns the log switch, if any.
    #[must_use]
    #[inline]
    pub const fn log(&self) -> Option<&LogSwitch> {
        self.log.as_ref()
    }

    /// Returns the upgrade switch, if any.
    #[must_use]
    #[inline]
    pub const fn upgrade(&self) -> Option<&UpgradeSwitch> {
        self.upgrade.as_ref()
    }

    /// Returns the custom switch, if any.
    #[must_use]
    #[inline]
    pub const fn custom(&self) -> Option<&CustomSwitch> {
        self.custom.as_ref()
    }

    /// Returns the repair switch, if any.
    #[must_use]
    #[inline]
    pub const fn repair(&self) -> Option<&RepairSwitch> {
        self.repair.as_ref()
    }

    /// Returns `true` if no switches are present.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::{switches, InstallerSwitches};
    ///
    /// let switches = InstallerSwitches::builder().build();
    /// assert!(switches.is_empty());
    ///
    /// let switches = InstallerSwitches::builder().maybe_silent("--silent".parse().ok()).build();
    /// assert!(!switches.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.silent.is_none()
            && self.silent_with_progress.is_none()
            && self.interactive.is_none()
            && self.install_location.is_none()
            && self.log.is_none()
            && self.upgrade.is_none()
            && self.custom.is_none()
            && self.repair.is_none()
    }
}

impl Default for InstallerSwitches {
    fn default() -> Self {
        Self::builder().build()
    }
}

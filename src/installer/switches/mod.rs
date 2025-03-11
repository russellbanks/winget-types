mod custom;
mod install_location;
mod interactive;
mod log;
mod repair;
mod silent;
mod silent_with_progress;
mod switch;
mod upgrade;

pub use super::switches::{
    custom::CustomSwitch, install_location::InstallLocationSwitch, interactive::InteractiveSwitch,
    log::LogSwitch, repair::RepairSwitch, silent::SilentSwitch,
    silent_with_progress::SilentWithProgressSwitch, upgrade::UpgradeSwitch,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct InstallerSwitches {
    /// Switches passed to the installer to provide a silent install experience.
    ///
    /// These would be used when the command `winget install <package> --silent` is executed.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub silent: Option<SilentSwitch>,

    /// Switches passed to the installer to provide a silent with progress install experience.
    ///
    /// This is intended to allow a progress indication to the user, and the indication may come
    /// from an installer UI dialogue, but it must not require user interaction to complete. The
    /// Windows Package Manager currently defaults to this install experience.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub silent_with_progress: Option<SilentWithProgressSwitch>,

    /// Switches passed to the installer to provide an interactive install experience.
    ///
    /// This is intended to allow a user to interact with the installer. These would be used when
    /// the command `winget install <package> --interactive` is executed.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub interactive: Option<InteractiveSwitch>,

    /// The path to install the package if the installer supports installing the package in a user
    /// configurable location.
    ///
    /// The `<INSTALLPATH>` token can be included in the switch value so the Windows Package Manager
    /// will replace the token with user provided path.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub install_location: Option<InstallLocationSwitch>,

    /// The path logs will be directed to if the installer supports specifying the log path in a
    /// user configurable location.
    ///
    /// The `<LOGPATH>` token can be included in the switch value so the Windows Package Manager
    /// will replace the token with user provided path.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub log: Option<LogSwitch>,

    /// The switches to be passed to the installer during an upgrade. This will happen only if the
    /// upgrade behavior is "install".
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub upgrade: Option<UpgradeSwitch>,

    /// Any switches the Windows Package Manager will pass to the installer in addition to `Silent`,
    /// `SilentWithProgress`, and `Interactive`.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub custom: Option<CustomSwitch>,

    /// The switches to be passed during the repair of an existing installation.
    ///
    /// This will be passed to the installer, `ModifyPath` ARP command, or Uninstaller ARP command
    /// depending on the `RepairBehavior` specified in the manifest.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub repair: Option<RepairSwitch>,
}

impl InstallerSwitches {
    const DEFAULT: Self = Self {
        silent: None,
        silent_with_progress: None,
        interactive: None,
        install_location: None,
        log: None,
        upgrade: None,
        custom: None,
        repair: None,
    };

    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self::DEFAULT
    }

    #[must_use]
    pub fn silent<T: AsRef<str>>(mut self, silent: T) -> Self {
        self.silent = silent.as_ref().parse().ok();
        self
    }

    #[must_use]
    pub fn silent_with_progress<T: AsRef<str>>(mut self, silent_with_progress: T) -> Self {
        self.silent_with_progress = silent_with_progress.as_ref().parse().ok();
        self
    }

    #[must_use]
    pub fn interactive<T: AsRef<str>>(mut self, interactive: T) -> Self {
        self.interactive = interactive.as_ref().parse().ok();
        self
    }

    #[must_use]
    pub fn install_location<T: AsRef<str>>(mut self, install_location: T) -> Self {
        self.install_location = install_location.as_ref().parse().ok();
        self
    }

    #[must_use]
    pub fn log<T: AsRef<str>>(mut self, log: T) -> Self {
        self.log = log.as_ref().parse().ok();
        self
    }

    #[must_use]
    pub fn upgrade<T: AsRef<str>>(mut self, upgrade: T) -> Self {
        self.upgrade = upgrade.as_ref().parse().ok();
        self
    }

    #[must_use]
    pub fn custom<T: AsRef<str>>(mut self, custom: T) -> Self {
        self.custom = custom.as_ref().parse().ok();
        self
    }

    #[must_use]
    pub fn repair<T: AsRef<str>>(mut self, repair: T) -> Self {
        self.repair = repair.as_ref().parse().ok();
        self
    }

    /// Returns `true` if no switches are present.
    ///
    /// # Examples
    ///
    /// ```
    /// use winget_types::installer::{switches, InstallerSwitches};
    ///
    /// let mut switches = InstallerSwitches::default();
    /// assert!(switches.is_empty());
    /// switches.silent = "--silent".parse().ok();
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
    }
}

impl Default for InstallerSwitches {
    fn default() -> Self {
        Self::DEFAULT
    }
}

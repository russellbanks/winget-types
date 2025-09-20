use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ReturnResponse {
    PackageInUse,
    PackageInUseByApplication,
    InstallInProgress,
    FileInUse,
    MissingDependency,
    DiskFull,
    InsufficientMemory,
    InvalidParameter,
    NoNetwork,
    ContactSupport,
    RebootRequiredToFinish,
    RebootRequiredForInstall,
    RebootInitiated,
    CancelledByUser,
    AlreadyInstalled,
    Downgrade,
    BlockedByPolicy,
    SystemNotSupported,
    Custom,
}

impl ReturnResponse {
    /// Returns the return response type as a static string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageInUse => "Package in use",
            Self::PackageInUseByApplication => "Package in use by application",
            Self::InstallInProgress => "Install in progress",
            Self::FileInUse => "File in use",
            Self::MissingDependency => "Missing dependency",
            Self::DiskFull => "Disk full",
            Self::InsufficientMemory => "Insufficient memory",
            Self::InvalidParameter => "Invalid parameter",
            Self::NoNetwork => "No network",
            Self::ContactSupport => "Contact support",
            Self::RebootRequiredToFinish => "Reboot required to finish",
            Self::RebootRequiredForInstall => "Reboot required to install",
            Self::RebootInitiated => "Reboot initiated",
            Self::CancelledByUser => "Cancelled by user",
            Self::AlreadyInstalled => "Already installed",
            Self::Downgrade => "Downgrade",
            Self::BlockedByPolicy => "Blocked by policy",
            Self::SystemNotSupported => "System not supported",
            Self::Custom => "Custom",
        }
    }
}

impl fmt::Display for ReturnResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

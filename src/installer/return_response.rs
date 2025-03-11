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

impl fmt::Display for ReturnResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PackageInUse => f.write_str("Package in use"),
            Self::PackageInUseByApplication => f.write_str("Package in use by application"),
            Self::InstallInProgress => f.write_str("Install in progress"),
            Self::FileInUse => f.write_str("File in use"),
            Self::MissingDependency => f.write_str("Missing dependency"),
            Self::DiskFull => f.write_str("Disk full"),
            Self::InsufficientMemory => f.write_str("Insufficient memory"),
            Self::InvalidParameter => f.write_str("Invalid parameter"),
            Self::NoNetwork => f.write_str("No network"),
            Self::ContactSupport => f.write_str("Contact support"),
            Self::RebootRequiredToFinish => f.write_str("Reboot required to finish"),
            Self::RebootRequiredForInstall => f.write_str("Reboot required to install"),
            Self::RebootInitiated => f.write_str("Reboot initiated"),
            Self::CancelledByUser => f.write_str("Cancelled by user"),
            Self::AlreadyInstalled => f.write_str("Already installed"),
            Self::Downgrade => f.write_str("Downgrade"),
            Self::BlockedByPolicy => f.write_str("Blocked by policy"),
            Self::SystemNotSupported => f.write_str("System not supported"),
            Self::Custom => f.write_str("Custom"),
        }
    }
}

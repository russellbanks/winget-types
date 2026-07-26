pub mod installer;
pub mod locale;
mod version;

pub use installer::InstallerManifest;
pub use locale::{DefaultLocaleManifest, LocaleManifest};
pub use version::VersionManifest;

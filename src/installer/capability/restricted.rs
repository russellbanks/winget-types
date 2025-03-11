use core::{fmt, str::FromStr};

use super::CapabilityError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RestrictedCapability {
    AccessoryManager,
    AllAppMods,
    AllowElevation,
    AppBroadcastServices,
    AppCaptureServices,
    AppCaptureSettings,
    AppDiagnostics,
    AppLicensing,
    AppointmentsSystem,
    AudioDeviceConfiguration,
    BackgroundMediaRecording,
    BackgroundSpatialPerception,
    BackgroundVoIP,
    BroadFileSystemAccess,
    CameraProcessingExtension,
    CellularDeviceControl,
    CellularDeviceIdentity,
    CellularMessaging,
    ChatSystem,
    ConfirmAppClose,
    ContactsSystem,
    CortanaPermissions,
    CortanaSpeechAccessory,
    CustomInstallActions,
    DevelopmentModeNetwork,
    DeviceManagementDmAccount,
    DeviceManagementEmailAccount,
    DeviceManagementFoundation,
    DeviceManagementWapSecurityPolicies,
    DevicePortalProvider,
    DeviceUnlock,
    DocumentsLibrary,
    DualSimTiles,
    Email,
    EmailSystem,
    EnterpriseAuthentication,
    EnterpriseCloudSSO,
    EnterpriseDataPolicy,
    EnterpriseDeviceLockdown,
    ExpandedResources,
    ExtendedBackgroundTaskTime,
    ExtendedExecutionBackgroundAudio,
    ExtendedExecutionCritical,
    ExtendedExecutionUnconstrained,
    FirstSignInSettings,
    GameBarServices,
    GameList,
    GameMonitor,
    InputForegroundObservation,
    InputInjectionBrokered,
    InputObservation,
    InputSuppression,
    InteropServices,
    LocalSystemServices,
    LocationHistory,
    LocationSystem,
    ModifiableApp,
    NetworkConnectionManagerProvisioning,
    NetworkDataPlanProvisioning,
    NetworkDataUsageManagement,
    NetworkingVpnProvider,
    OemDeployment,
    OemPublicDirectory,
    OneProcessVoIP,
    PackagedServices,
    PackageManagement,
    PackagePolicySystem,
    PackageQuery,
    PackageWriteRedirectionCompatibilityShim,
    PhoneCallHistory,
    PhoneCallHistorySystem,
    PhoneLineTransportManagement,
    PreviewInkWorkspace,
    PreviewPenWorkspace,
    PreviewStore,
    PreviewUiComposition,
    ProtectedApp,
    RemotePassportAuthentication,
    RunFullTrust,
    ScreenDuplication,
    SecondaryAuthenticationFactor,
    SecureAssessment,
    SharedUserCertificates,
    SlapiQueryLicenseValue,
    SmBIOS,
    SmsSend,
    StartScreenManagement,
    StoreLicenseManagement,
    TargetedContent,
    TeamEditionDeviceCredential,
    TeamEditionExperience,
    TeamEditionView,
    UIAccess,
    UiAutomation,
    UnvirtualizedResources,
    UserDataAccountsProvider,
    UserDataSystem,
    UserPrincipalName,
    UserSystemId,
    WalletSystem,
    XboxAccessoryManagement,
}

impl AsRef<str> for RestrictedCapability {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl RestrictedCapability {
    pub const MAX_LEN: usize = 40;

    #[expect(clippy::too_many_lines, reason = "Necessary for an exhaustive match")]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccessoryManager => "accessoryManager",
            Self::AllAppMods => "allAppMods",
            Self::AllowElevation => "allowElevation",
            Self::AppBroadcastServices => "appBroadcastServices",
            Self::AppCaptureServices => "appCaptureServices",
            Self::AppCaptureSettings => "appCaptureSettings",
            Self::AppDiagnostics => "appDiagnostics",
            Self::AppLicensing => "appLicensing",
            Self::AppointmentsSystem => "appointmentsSystem",
            Self::AudioDeviceConfiguration => "audioDeviceConfiguration",
            Self::BackgroundMediaRecording => "backgroundMediaRecording",
            Self::BackgroundSpatialPerception => "backgroundSpatialPerception",
            Self::BackgroundVoIP => "backgroundVoIP",
            Self::BroadFileSystemAccess => "broadFileSystemAccess",
            Self::CameraProcessingExtension => "cameraProcessingExtension",
            Self::CellularDeviceControl => "cellularDeviceControl",
            Self::CellularDeviceIdentity => "cellularDeviceIdentity",
            Self::CellularMessaging => "cellularMessaging",
            Self::ChatSystem => "chatSystem",
            Self::ConfirmAppClose => "confirmAppClose",
            Self::ContactsSystem => "contactsSystem",
            Self::CortanaPermissions => "cortanaPermissions",
            Self::CortanaSpeechAccessory => "cortanaSpeechAccessory",
            Self::CustomInstallActions => "customInstallActions",
            Self::DevelopmentModeNetwork => "developmentModeNetwork",
            Self::DeviceManagementDmAccount => "deviceManagementDmAccount",
            Self::DeviceManagementEmailAccount => "deviceManagementEmailAccount",
            Self::DeviceManagementFoundation => "deviceManagementFoundation",
            Self::DeviceManagementWapSecurityPolicies => "deviceManagementWapSecurityPolicies",
            Self::DevicePortalProvider => "devicePortalProvider",
            Self::DeviceUnlock => "deviceUnlock",
            Self::DocumentsLibrary => "documentsLibrary",
            Self::DualSimTiles => "dualSimTiles",
            Self::Email => "email",
            Self::EmailSystem => "emailSystem",
            Self::EnterpriseAuthentication => "enterpriseAuthentication",
            Self::EnterpriseCloudSSO => "enterpriseCloudSSO",
            Self::EnterpriseDataPolicy => "enterpriseDataPolicy",
            Self::EnterpriseDeviceLockdown => "enterpriseDeviceLockdown",
            Self::ExpandedResources => "expandedResources",
            Self::ExtendedBackgroundTaskTime => "extendedBackgroundTaskTime",
            Self::ExtendedExecutionBackgroundAudio => "extendedExecutionBackgroundAudio",
            Self::ExtendedExecutionCritical => "extendedExecutionCritical",
            Self::ExtendedExecutionUnconstrained => "extendedExecutionUnconstrained",
            Self::FirstSignInSettings => "firstSignInSettings",
            Self::GameBarServices => "gameBarServices",
            Self::GameList => "gameList",
            Self::GameMonitor => "gameMonitor",
            Self::InputForegroundObservation => "inputForegroundObservation",
            Self::InputInjectionBrokered => "inputInjectionBrokered",
            Self::InputObservation => "inputObservation",
            Self::InputSuppression => "inputSuppression",
            Self::InteropServices => "interopServices",
            Self::LocalSystemServices => "localSystemServices",
            Self::LocationHistory => "locationHistory",
            Self::LocationSystem => "locationSystem",
            Self::ModifiableApp => "modifiableApp",
            Self::NetworkConnectionManagerProvisioning => "networkConnectionManagerProvisioning",
            Self::NetworkDataPlanProvisioning => "networkDataPlanProvisioning",
            Self::NetworkDataUsageManagement => "networkDataUsageManagement",
            Self::NetworkingVpnProvider => "networkingVpnProvider",
            Self::OemDeployment => "oemDeployment",
            Self::OemPublicDirectory => "oemPublicDirectory",
            Self::OneProcessVoIP => "oneProcessVoIP",
            Self::PackagedServices => "packagedServices",
            Self::PackageManagement => "packageManagement",
            Self::PackagePolicySystem => "packagePolicySystem",
            Self::PackageQuery => "packageQuery",
            Self::PackageWriteRedirectionCompatibilityShim => {
                "packageWriteRedirectionCompatibilityShim"
            }
            Self::PhoneCallHistory => "phoneCallHistory",
            Self::PhoneCallHistorySystem => "phoneCallHistorySystem",
            Self::PhoneLineTransportManagement => "phoneLineTransportManagement",
            Self::PreviewInkWorkspace => "previewInkWorkspace",
            Self::PreviewPenWorkspace => "previewPenWorkspace",
            Self::PreviewStore => "previewStore",
            Self::PreviewUiComposition => "previewUiComposition",
            Self::ProtectedApp => "protectedApp",
            Self::RemotePassportAuthentication => "remotePassportAuthentication",
            Self::RunFullTrust => "runFullTrust",
            Self::ScreenDuplication => "screenDuplication",
            Self::SecondaryAuthenticationFactor => "secondaryAuthenticationFactor",
            Self::SecureAssessment => "secureAssessment",
            Self::SharedUserCertificates => "sharedUserCertificates",
            Self::SlapiQueryLicenseValue => "slapiQueryLicenseValue",
            Self::SmBIOS => "smbios",
            Self::SmsSend => "smsSend",
            Self::StartScreenManagement => "startScreenManagement",
            Self::StoreLicenseManagement => "storeLicenseManagement",
            Self::TargetedContent => "targetedContent",
            Self::TeamEditionDeviceCredential => "teamEditionDeviceCredential",
            Self::TeamEditionExperience => "teamEditionExperience",
            Self::TeamEditionView => "teamEditionView",
            Self::UIAccess => "uiAccess",
            Self::UiAutomation => "uiAutomation",
            Self::UnvirtualizedResources => "unvirtualizedResources",
            Self::UserDataAccountsProvider => "userDataAccountsProvider",
            Self::UserDataSystem => "userDataSystem",
            Self::UserPrincipalName => "userPrincipalName",
            Self::UserSystemId => "userSystemId",
            Self::WalletSystem => "walletSystem",
            Self::XboxAccessoryManagement => "xboxAccessoryManagement",
        }
    }
}

impl fmt::Display for RestrictedCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for RestrictedCapability {
    type Err = CapabilityError;

    #[expect(
        clippy::too_many_lines,
        reason = "Necessary for an exhaustive from_str"
    )]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Self::Err::Empty);
        }

        match s {
            "accessoryManager" => Ok(Self::AccessoryManager),
            "allAppMods" => Ok(Self::AllAppMods),
            "allowElevation" => Ok(Self::AllowElevation),
            "appBroadcastServices" => Ok(Self::AppBroadcastServices),
            "appCaptureServices" => Ok(Self::AppCaptureServices),
            "appCaptureSettings" => Ok(Self::AppCaptureSettings),
            "appDiagnostics" => Ok(Self::AppDiagnostics),
            "appLicensing" => Ok(Self::AppLicensing),
            "appointmentsSystem" => Ok(Self::AppointmentsSystem),
            "audioDeviceConfiguration" => Ok(Self::AudioDeviceConfiguration),
            "backgroundMediaRecording" => Ok(Self::BackgroundMediaRecording),
            "backgroundSpatialPerception" => Ok(Self::BackgroundSpatialPerception),
            "backgroundVoIP" => Ok(Self::BackgroundVoIP),
            "broadFileSystemAccess" => Ok(Self::BroadFileSystemAccess),
            "cameraProcessingExtension" => Ok(Self::CameraProcessingExtension),
            "cellularDeviceControl" => Ok(Self::CellularDeviceControl),
            "cellularDeviceIdentity" => Ok(Self::CellularDeviceIdentity),
            "cellularMessaging" => Ok(Self::CellularMessaging),
            "chatSystem" => Ok(Self::ChatSystem),
            "confirmAppClose" => Ok(Self::ConfirmAppClose),
            "contactsSystem" => Ok(Self::ContactsSystem),
            "cortanaPermissions" => Ok(Self::CortanaPermissions),
            "cortanaSpeechAccessory" => Ok(Self::CortanaSpeechAccessory),
            "customInstallActions" => Ok(Self::CustomInstallActions),
            "developmentModeNetwork" => Ok(Self::DevelopmentModeNetwork),
            "deviceManagementDmAccount" => Ok(Self::DeviceManagementDmAccount),
            "deviceManagementEmailAccount" => Ok(Self::DeviceManagementEmailAccount),
            "deviceManagementFoundation" => Ok(Self::DeviceManagementFoundation),
            "deviceManagementWapSecurityPolicies" => Ok(Self::DeviceManagementWapSecurityPolicies),
            "devicePortalProvider" => Ok(Self::DevicePortalProvider),
            "deviceUnlock" => Ok(Self::DeviceUnlock),
            "documentsLibrary" => Ok(Self::DocumentsLibrary),
            "dualSimTiles" => Ok(Self::DualSimTiles),
            "email" => Ok(Self::Email),
            "emailSystem" => Ok(Self::EmailSystem),
            "enterpriseAuthentication" => Ok(Self::EnterpriseAuthentication),
            "enterpriseCloudSSO" => Ok(Self::EnterpriseCloudSSO),
            "enterpriseDataPolicy" => Ok(Self::EnterpriseDataPolicy),
            "enterpriseDeviceLockdown" => Ok(Self::EnterpriseDeviceLockdown),
            "expandedResources" => Ok(Self::ExpandedResources),
            "extendedBackgroundTaskTime" => Ok(Self::ExtendedBackgroundTaskTime),
            "extendedExecutionBackgroundAudio" => Ok(Self::ExtendedExecutionBackgroundAudio),
            "extendedExecutionCritical" => Ok(Self::ExtendedExecutionCritical),
            "extendedExecutionUnconstrained" => Ok(Self::ExtendedExecutionUnconstrained),
            "firstSignInSettings" => Ok(Self::FirstSignInSettings),
            "gameBarServices" => Ok(Self::GameBarServices),
            "gameList" => Ok(Self::GameList),
            "gameMonitor" => Ok(Self::GameMonitor),
            "inputForegroundObservation" => Ok(Self::InputForegroundObservation),
            "inputInjectionBrokered" => Ok(Self::InputInjectionBrokered),
            "inputObservation" => Ok(Self::InputObservation),
            "inputSuppression" => Ok(Self::InputSuppression),
            "interopServices" => Ok(Self::InteropServices),
            "localSystemServices" => Ok(Self::LocalSystemServices),
            "locationHistory" => Ok(Self::LocationHistory),
            "locationSystem" => Ok(Self::LocationSystem),
            "modifiableApp" => Ok(Self::ModifiableApp),
            "networkConnectionManagerProvisioning" => {
                Ok(Self::NetworkConnectionManagerProvisioning)
            }
            "networkDataPlanProvisioning" => Ok(Self::NetworkDataPlanProvisioning),
            "networkDataUsageManagement" => Ok(Self::NetworkDataUsageManagement),
            "networkingVpnProvider" => Ok(Self::NetworkingVpnProvider),
            "oemDeployment" => Ok(Self::OemDeployment),
            "oemPublicDirectory" => Ok(Self::OemPublicDirectory),
            "oneProcessVoIP" => Ok(Self::OneProcessVoIP),
            "packagedServices" => Ok(Self::PackagedServices),
            "packageManagement" => Ok(Self::PackageManagement),
            "packagePolicySystem" => Ok(Self::PackagePolicySystem),
            "packageQuery" => Ok(Self::PackageQuery),
            "packageWriteRedirectionCompatibilityShim" => {
                Ok(Self::PackageWriteRedirectionCompatibilityShim)
            }
            "phoneCallHistory" => Ok(Self::PhoneCallHistory),
            "phoneCallHistorySystem" => Ok(Self::PhoneCallHistorySystem),
            "phoneLineTransportManagement" => Ok(Self::PhoneLineTransportManagement),
            "previewInkWorkspace" => Ok(Self::PreviewInkWorkspace),
            "previewPenWorkspace" => Ok(Self::PreviewPenWorkspace),
            "previewStore" => Ok(Self::PreviewStore),
            "previewUiComposition" => Ok(Self::PreviewUiComposition),
            "protectedApp" => Ok(Self::ProtectedApp),
            "remotePassportAuthentication" => Ok(Self::RemotePassportAuthentication),
            "runFullTrust" => Ok(Self::RunFullTrust),
            "screenDuplication" => Ok(Self::ScreenDuplication),
            "secondaryAuthenticationFactor" => Ok(Self::SecondaryAuthenticationFactor),
            "secureAssessment" => Ok(Self::SecureAssessment),
            "sharedUserCertificates" => Ok(Self::SharedUserCertificates),
            "slapiQueryLicenseValue" => Ok(Self::SlapiQueryLicenseValue),
            "smbios" => Ok(Self::SmBIOS),
            "smsSend" => Ok(Self::SmsSend),
            "startScreenManagement" => Ok(Self::StartScreenManagement),
            "storeLicenseManagement" => Ok(Self::StoreLicenseManagement),
            "targetedContent" => Ok(Self::TargetedContent),
            "teamEditionDeviceCredential" => Ok(Self::TeamEditionDeviceCredential),
            "teamEditionExperience" => Ok(Self::TeamEditionExperience),
            "teamEditionView" => Ok(Self::TeamEditionView),
            "uiAccess" => Ok(Self::UIAccess),
            "uiAutomation" => Ok(Self::UiAutomation),
            "unvirtualizedResources" => Ok(Self::UnvirtualizedResources),
            "userDataAccountsProvider" => Ok(Self::UserDataAccountsProvider),
            "userDataSystem" => Ok(Self::UserDataSystem),
            "userPrincipalName" => Ok(Self::UserPrincipalName),
            "userSystemId" => Ok(Self::UserSystemId),
            "walletSystem" => Ok(Self::WalletSystem),
            "xboxAccessoryManagement" => Ok(Self::XboxAccessoryManagement),
            _ => Err(Self::Err::Unknown(
                s.parse::<heapless::String<{ Self::MAX_LEN }>>()
                    .map_err(|()| Self::Err::TooLong(s.len()))?,
            )),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RestrictedCapability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RestrictedCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RestrictedCapabilityVisitor;

        impl serde::de::Visitor<'_> for RestrictedCapabilityVisitor {
            type Value = RestrictedCapability;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a restricted capability string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value.parse::<Self::Value>().map_err(E::custom)
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let utf8 = core::str::from_utf8(value).map_err(E::custom)?;
                self.visit_str(utf8)
            }
        }

        deserializer.deserialize_str(RestrictedCapabilityVisitor)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{CapabilityError, RestrictedCapability};

    #[rstest]
    #[case("enterpriseAuthentication")]
    #[case("enterpriseDataPolicy")]
    #[case("sharedUserCertificates")]
    #[case("documentsLibrary")]
    #[case("appCaptureSettings")]
    #[case("cellularDeviceControl")]
    #[case("cellularDeviceIdentity")]
    #[case("cellularMessaging")]
    #[case("deviceUnlock")]
    #[case("dualSimTiles")]
    #[case("enterpriseDeviceLockdown")]
    #[case("inputInjectionBrokered")]
    #[case("inputObservation")]
    #[case("inputSuppression")]
    #[case("networkingVpnProvider")]
    #[case("packageManagement")]
    #[case("packageQuery")]
    #[case("screenDuplication")]
    #[case("userPrincipalName")]
    #[case("walletSystem")]
    #[case("locationHistory")]
    #[case("confirmAppClose")]
    #[case("phoneCallHistory")]
    #[case("appointmentsSystem")]
    #[case("chatSystem")]
    #[case("contactsSystem")]
    #[case("email")]
    #[case("emailSystem")]
    #[case("phoneCallHistorySystem")]
    #[case("smsSend")]
    #[case("userDataSystem")]
    #[case("previewStore")]
    #[case("firstSignInSettings")]
    #[case("teamEditionExperience")]
    #[case("remotePassportAuthentication")]
    #[case("previewUiComposition")]
    #[case("secureAssessment")]
    #[case("networkConnectionManagerProvisioning")]
    #[case("networkDataPlanProvisioning")]
    #[case("slapiQueryLicenseValue")]
    #[case("extendedBackgroundTaskTime")]
    #[case("extendedExecutionBackgroundAudio")]
    #[case("extendedExecutionCritical")]
    #[case("extendedExecutionUnconstrained")]
    #[case("deviceManagementDmAccount")]
    #[case("deviceManagementFoundation")]
    #[case("deviceManagementWapSecurityPolicies")]
    #[case("deviceManagementEmailAccount")]
    #[case("packagePolicySystem")]
    #[case("gameList")]
    #[case("xboxAccessoryManagement")]
    #[case("cortanaSpeechAccessory")]
    #[case("accessoryManager")]
    #[case("interopServices")]
    #[case("inputForegroundObservation")]
    #[case("oemDeployment")]
    #[case("oemPublicDirectory")]
    #[case("appLicensing")]
    #[case("locationSystem")]
    #[case("userDataAccountsProvider")]
    #[case("previewPenWorkspace")]
    #[case("secondaryAuthenticationFactor")]
    #[case("storeLicenseManagement")]
    #[case("userSystemId")]
    #[case("targetedContent")]
    #[case("uiAutomation")]
    #[case("gameBarServices")]
    #[case("appCaptureServices")]
    #[case("appBroadcastServices")]
    #[case("audioDeviceConfiguration")]
    #[case("backgroundMediaRecording")]
    #[case("previewInkWorkspace")]
    #[case("startScreenManagement")]
    #[case("cortanaPermissions")]
    #[case("allAppMods")]
    #[case("expandedResources")]
    #[case("protectedApp")]
    #[case("gameMonitor")]
    #[case("appDiagnostics")]
    #[case("devicePortalProvider")]
    #[case("enterpriseCloudSSO")]
    #[case("backgroundVoIP")]
    #[case("oneProcessVoIP")]
    #[case("developmentModeNetwork")]
    #[case("broadFileSystemAccess")]
    #[case("smbios")]
    #[case("runFullTrust")]
    #[case("allowElevation")]
    #[case("teamEditionDeviceCredential")]
    #[case("teamEditionView")]
    #[case("cameraProcessingExtension")]
    #[case("networkDataUsageManagement")]
    #[case("phoneLineTransportManagement")]
    #[case("unvirtualizedResources")]
    #[case("modifiableApp")]
    #[case("packageWriteRedirectionCompatibilityShim")]
    #[case("customInstallActions")]
    #[case("packagedServices")]
    #[case("localSystemServices")]
    #[case("backgroundSpatialPerception")]
    #[case("uiAccess")]
    fn valid_restricted_capability(#[case] restricted_capability: &str) {
        assert!(
            restricted_capability
                .parse::<RestrictedCapability>()
                .is_ok()
        );
    }

    #[test]
    fn invalid_restricted_capability() {
        assert_eq!(
            "".parse::<RestrictedCapability>().err(),
            Some(CapabilityError::Empty)
        );
    }
}

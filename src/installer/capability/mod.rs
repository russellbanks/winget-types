mod restricted;

use core::{fmt, str::FromStr};

use heapless::String;
pub use restricted::RestrictedCapability;
use thiserror::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Capability {
    Activity,
    AllJoyn,
    Appointments,
    BackgroundMediaPlayback,
    BlockedChatMessages,
    Bluetooth,
    Chat,
    CodeGeneration,
    Contacts,
    GazeInput,
    GlobalMediaControl,
    GraphicsCapture,
    GraphicsCaptureProgrammatic,
    GraphicsCaptureWithoutBorder,
    HumanInterfaceDevice,
    HumanPresence,
    InternetClient,
    InternetClientServer,
    Location,
    LowLevel,
    LowLevelDevices,
    Microphone,
    MusicLibrary,
    Objects3D,
    Optical,
    PhoneCall,
    PhoneCallHistoryPublic,
    PicturesLibrary,
    PointOfService,
    PrivateNetworkClientServer,
    Proximity,
    Radios,
    RecordedCallsFolder,
    RemoteSystem,
    RemovableStorage,
    SerialCommunication,
    SpatialPerception,
    SystemManagement,
    Usb,
    UserAccountInformation,
    UserDataTasks,
    UserNotificationListener,
    VideosLibrary,
    VoipCall,
    Webcam,
    WiFiControl,
}

impl Capability {
    pub const MAX_LEN: usize = 40;

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Activity => "activity",
            Self::AllJoyn => "allJoyn",
            Self::Appointments => "appointments",
            Self::BackgroundMediaPlayback => "backgroundMediaPlayback",
            Self::BlockedChatMessages => "blockedChatMessages",
            Self::Bluetooth => "bluetooth",
            Self::Chat => "chat",
            Self::CodeGeneration => "codeGeneration",
            Self::Contacts => "contacts",
            Self::GazeInput => "gazeInput",
            Self::GlobalMediaControl => "globalMediaControl",
            Self::GraphicsCapture => "graphicsCapture",
            Self::GraphicsCaptureProgrammatic => "graphicsCaptureProgrammatic",
            Self::GraphicsCaptureWithoutBorder => "graphicsCaptureWithoutBorder",
            Self::HumanInterfaceDevice => "humaninterfacedevice",
            Self::HumanPresence => "humanPresence",
            Self::InternetClient => "internetClient",
            Self::InternetClientServer => "internetClientServer",
            Self::Location => "location",
            Self::LowLevel => "lowLevel",
            Self::LowLevelDevices => "lowLevelDevices",
            Self::Microphone => "microphone",
            Self::MusicLibrary => "musicLibrary",
            Self::Objects3D => "objects3D",
            Self::Optical => "optical",
            Self::PhoneCall => "phoneCall",
            Self::PhoneCallHistoryPublic => "phoneCallHistoryPublic",
            Self::PicturesLibrary => "picturesLibrary",
            Self::PointOfService => "pointOfService",
            Self::PrivateNetworkClientServer => "privateNetworkClientServer",
            Self::Proximity => "proximity",
            Self::Radios => "radios",
            Self::RecordedCallsFolder => "recordedCallsFolder",
            Self::RemoteSystem => "remoteSystem",
            Self::RemovableStorage => "removableStorage",
            Self::SerialCommunication => "serialcommunication",
            Self::SpatialPerception => "spatialPerception",
            Self::SystemManagement => "systemManagement",
            Self::Usb => "usb",
            Self::UserAccountInformation => "userAccountInformation",
            Self::UserDataTasks => "userDataTasks",
            Self::UserNotificationListener => "userNotificationListener",
            Self::VideosLibrary => "videosLibrary",
            Self::VoipCall => "voipCall",
            Self::Webcam => "webcam",
            Self::WiFiControl => "wiFiControl",
        }
    }
}

impl AsRef<str> for Capability {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CapabilityError {
    #[error("Capability must not be empty")]
    Empty,
    #[error(
        "Capability must not have more than {} ASCII characters but has {_0}",
        Capability::MAX_LEN
    )]
    TooLong(usize),
    #[error(r#""{_0}" is not a known capability"#)]
    Unknown(String<40>),
}

impl FromStr for Capability {
    type Err = CapabilityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Self::Err::Empty);
        }

        match s {
            "activity" => Ok(Self::Activity),
            "allJoyn" => Ok(Self::AllJoyn),
            "appointments" => Ok(Self::Appointments),
            "backgroundMediaPlayback" => Ok(Self::BackgroundMediaPlayback),
            "blockedChatMessages" => Ok(Self::BlockedChatMessages),
            "bluetooth" => Ok(Self::Bluetooth),
            "chat" => Ok(Self::Chat),
            "codeGeneration" => Ok(Self::CodeGeneration),
            "contacts" => Ok(Self::Contacts),
            "gazeInput" => Ok(Self::GazeInput),
            "globalMediaControl" => Ok(Self::GlobalMediaControl),
            "graphicsCapture" => Ok(Self::GraphicsCapture),
            "graphicsCaptureProgrammatic" => Ok(Self::GraphicsCaptureProgrammatic),
            "graphicsCaptureWithoutBorder" => Ok(Self::GraphicsCaptureWithoutBorder),
            "humaninterfacedevice" => Ok(Self::HumanInterfaceDevice),
            "humanPresence" => Ok(Self::HumanPresence),
            "internetClient" => Ok(Self::InternetClient),
            "internetClientServer" => Ok(Self::InternetClientServer),
            "location" => Ok(Self::Location),
            "lowLevel" => Ok(Self::LowLevel),
            "lowLevelDevices" => Ok(Self::LowLevelDevices),
            "microphone" => Ok(Self::Microphone),
            "musicLibrary" => Ok(Self::MusicLibrary),
            "objects3D" => Ok(Self::Objects3D),
            "optical" => Ok(Self::Optical),
            "phoneCall" => Ok(Self::PhoneCall),
            "phoneCallHistoryPublic" => Ok(Self::PhoneCallHistoryPublic),
            "picturesLibrary" => Ok(Self::PicturesLibrary),
            "pointOfService" => Ok(Self::PointOfService),
            "privateNetworkClientServer" => Ok(Self::PrivateNetworkClientServer),
            "proximity" => Ok(Self::Proximity),
            "radios" => Ok(Self::Radios),
            "recordedCallsFolder" => Ok(Self::RecordedCallsFolder),
            "remoteSystem" => Ok(Self::RemoteSystem),
            "removableStorage" => Ok(Self::RemovableStorage),
            "serialcommunication" => Ok(Self::SerialCommunication),
            "spatialPerception" => Ok(Self::SpatialPerception),
            "systemManagement" => Ok(Self::SystemManagement),
            "usb" => Ok(Self::Usb),
            "userAccountInformation" => Ok(Self::UserAccountInformation),
            "userDataTasks" => Ok(Self::UserDataTasks),
            "userNotificationListener" => Ok(Self::UserNotificationListener),
            "videosLibrary" => Ok(Self::VideosLibrary),
            "voipCall" => Ok(Self::VoipCall),
            "webcam" => Ok(Self::Webcam),
            "wiFiControl" => Ok(Self::WiFiControl),
            _ => Err(Self::Err::Unknown(
                s.parse::<String<{ Self::MAX_LEN }>>()
                    .map_err(|()| Self::Err::TooLong(s.len()))?,
            )),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Capability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Capability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CapabilityVisitor;

        impl serde::de::Visitor<'_> for CapabilityVisitor {
            type Value = Capability;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a capability string")
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

        deserializer.deserialize_str(CapabilityVisitor)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{Capability, CapabilityError};

    #[rstest]
    #[case("musicLibrary")]
    #[case("picturesLibrary")]
    #[case("videosLibrary")]
    #[case("removableStorage")]
    #[case("internetClient")]
    #[case("internetClientServer")]
    #[case("privateNetworkClientServer")]
    #[case("contacts")]
    #[case("codeGeneration")]
    #[case("allJoyn")]
    #[case("phoneCall")]
    #[case("phoneCallHistoryPublic")]
    #[case("recordedCallsFolder")]
    #[case("userAccountInformation")]
    #[case("objects3D")]
    #[case("chat")]
    #[case("blockedChatMessages")]
    #[case("lowLevelDevices")]
    #[case("systemManagement")]
    #[case("backgroundMediaPlayback")]
    #[case("remoteSystem")]
    #[case("spatialPerception")]
    #[case("globalMediaControl")]
    #[case("graphicsCapture")]
    #[case("graphicsCaptureWithoutBorder")]
    #[case("graphicsCaptureProgrammatic")]
    #[case("userDataTasks")]
    #[case("userNotificationListener")]
    fn valid_general_capability(#[case] capability: &str) {
        assert!(capability.parse::<Capability>().is_ok());
    }

    #[rstest]
    #[case("location")]
    #[case("microphone")]
    #[case("proximity")]
    #[case("webcam")]
    #[case("usb")]
    #[case("humaninterfacedevice")]
    #[case("pointOfService")]
    #[case("bluetooth")]
    #[case("wiFiControl")]
    #[case("radios")]
    #[case("optical")]
    #[case("activity")]
    #[case("humanPresence")]
    #[case("serialcommunication")]
    #[case("gazeInput")]
    #[case("lowLevel")]
    fn valid_device_capability(#[case] capability: &str) {
        assert!(capability.parse::<Capability>().is_ok());
    }

    #[test]
    fn invalid_capability() {
        assert_eq!("".parse::<Capability>().err(), Some(CapabilityError::Empty));
    }
}

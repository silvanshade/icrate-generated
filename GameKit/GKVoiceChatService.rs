//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKVoiceChatService")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GKVoiceChatService;
}

#[cfg(feature = "GameKit_GKVoiceChatService")]
unsafe impl NSObjectProtocol for GKVoiceChatService {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKVoiceChatService")]
    pub type GKVoiceChatService;

    #[objc2::method(sel = "defaultVoiceChatService", managed = "Other")]
    pub unsafe fn defaultVoiceChatService() -> Option<Id<GKVoiceChatService>>;

    #[objc2::method(sel = "isVoIPAllowed")]
    pub unsafe fn isVoIPAllowed() -> bool;

    #[objc2::method(sel = "client", managed = "Other")]
    pub unsafe fn client(&self) -> Option<Id<ProtocolObject<dyn GKVoiceChatClient>>>;

    #[objc2::method(sel = "setClient:")]
    pub unsafe fn setClient(&self, client: Option<&ProtocolObject<dyn GKVoiceChatClient>>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "stopVoiceChatWithParticipantID:")]
    pub unsafe fn stopVoiceChatWithParticipantID(&self, participant_id: Option<&NSString>);

    #[objc2::method(sel = "denyCallID:")]
    pub unsafe fn denyCallID(&self, call_id: NSInteger);

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "receivedRealTimeData:fromParticipantID:")]
    pub unsafe fn receivedRealTimeData_fromParticipantID(
        &self,
        audio: Option<&NSData>,
        participant_id: Option<&NSString>,
    );

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "receivedData:fromParticipantID:")]
    pub unsafe fn receivedData_fromParticipantID(
        &self,
        arbitrary_data: Option<&NSData>,
        participant_id: Option<&NSString>,
    );

    #[objc2::method(sel = "isMicrophoneMuted")]
    pub unsafe fn isMicrophoneMuted(&self) -> bool;

    #[objc2::method(sel = "setMicrophoneMuted:")]
    pub unsafe fn setMicrophoneMuted(&self, microphone_muted: bool);

    #[objc2::method(sel = "remoteParticipantVolume")]
    pub unsafe fn remoteParticipantVolume(&self) -> c_float;

    #[objc2::method(sel = "setRemoteParticipantVolume:")]
    pub unsafe fn setRemoteParticipantVolume(&self, remote_participant_volume: c_float);

    #[objc2::method(sel = "isOutputMeteringEnabled")]
    pub unsafe fn isOutputMeteringEnabled(&self) -> bool;

    #[objc2::method(sel = "setOutputMeteringEnabled:")]
    pub unsafe fn setOutputMeteringEnabled(&self, output_metering_enabled: bool);

    #[objc2::method(sel = "isInputMeteringEnabled")]
    pub unsafe fn isInputMeteringEnabled(&self) -> bool;

    #[objc2::method(sel = "setInputMeteringEnabled:")]
    pub unsafe fn setInputMeteringEnabled(&self, input_metering_enabled: bool);

    #[objc2::method(sel = "outputMeterLevel")]
    pub unsafe fn outputMeterLevel(&self) -> c_float;

    #[objc2::method(sel = "inputMeterLevel")]
    pub unsafe fn inputMeterLevel(&self) -> c_float;
}

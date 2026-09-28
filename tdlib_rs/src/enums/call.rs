//!
//! TDLib `call` domain enums.
//!
//! Types, enums, and functions for 1-on-1 calls, group calls, and live video chats.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// Describes the reason why a call was discarded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallDiscardReason {
    /// The call wasn't discarded, or the reason is unknown
    #[serde(rename(serialize = "callDiscardReasonEmpty", deserialize = "callDiscardReasonEmpty"))]
    Empty,
    /// The call was ended before the conversation started. It was canceled by the caller or missed by the other party
    #[serde(rename(serialize = "callDiscardReasonMissed", deserialize = "callDiscardReasonMissed"))]
    Missed,
    /// The call was ended before the conversation started. It was declined by the other party
    #[serde(rename(serialize = "callDiscardReasonDeclined", deserialize = "callDiscardReasonDeclined"))]
    Declined,
    /// The call was ended during the conversation because the users were disconnected
    #[serde(rename(serialize = "callDiscardReasonDisconnected", deserialize = "callDiscardReasonDisconnected"))]
    Disconnected,
    /// The call was ended because one of the parties hung up
    #[serde(rename(serialize = "callDiscardReasonHungUp", deserialize = "callDiscardReasonHungUp"))]
    HungUp,
    /// The call was ended because it has been upgraded to a group call
    #[serde(rename(serialize = "callDiscardReasonUpgradeToGroupCall", deserialize = "callDiscardReasonUpgradeToGroupCall"))]
    UpgradeToGroupCall(Box<crate::types::CallDiscardReasonUpgradeToGroupCall>),
}

impl CallDiscardReason {
    /// Convenience constructor to create a [`CallDiscardReason::UpgradeToGroupCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgrade_to_group_call(val: crate::types::CallDiscardReasonUpgradeToGroupCall) -> Self {
        Self::UpgradeToGroupCall(Box::new(val))
    }

}

/// Converts a [`crate::types::CallDiscardReasonUpgradeToGroupCall`] into [`CallDiscardReason`].
impl From<crate::types::CallDiscardReasonUpgradeToGroupCall> for CallDiscardReason {
    fn from(val: crate::types::CallDiscardReasonUpgradeToGroupCall) -> Self {
        Self::UpgradeToGroupCall(Box::new(val))
    }
}

/// TDLib `CallProtocol` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallProtocol {
    /// Specifies the supported call protocols
    #[serde(rename(serialize = "callProtocol", deserialize = "callProtocol"))]
    CallProtocol(Box<crate::types::CallProtocol>),
}

impl CallProtocol {
    /// Convenience constructor to create a [`CallProtocol::CallProtocol`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn call_protocol(val: crate::types::CallProtocol) -> Self {
        Self::CallProtocol(Box::new(val))
    }

}

/// Converts a [`crate::types::CallProtocol`] into [`CallProtocol`].
impl From<crate::types::CallProtocol> for CallProtocol {
    fn from(val: crate::types::CallProtocol) -> Self {
        Self::CallProtocol(Box::new(val))
    }
}

/// Describes the type of call server
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallServerType {
    /// A Telegram call reflector
    #[serde(rename(serialize = "callServerTypeTelegramReflector", deserialize = "callServerTypeTelegramReflector"))]
    TelegramReflector(Box<crate::types::CallServerTypeTelegramReflector>),
    /// A WebRTC server
    #[serde(rename(serialize = "callServerTypeWebrtc", deserialize = "callServerTypeWebrtc"))]
    Webrtc(Box<crate::types::CallServerTypeWebrtc>),
}

impl CallServerType {
    /// Convenience constructor to create a [`CallServerType::TelegramReflector`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn telegram_reflector(val: crate::types::CallServerTypeTelegramReflector) -> Self {
        Self::TelegramReflector(Box::new(val))
    }

    /// Convenience constructor to create a [`CallServerType::Webrtc`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn webrtc(val: crate::types::CallServerTypeWebrtc) -> Self {
        Self::Webrtc(Box::new(val))
    }

}

/// Converts a [`crate::types::CallServerTypeTelegramReflector`] into [`CallServerType`].
impl From<crate::types::CallServerTypeTelegramReflector> for CallServerType {
    fn from(val: crate::types::CallServerTypeTelegramReflector) -> Self {
        Self::TelegramReflector(Box::new(val))
    }
}

/// Converts a [`crate::types::CallServerTypeWebrtc`] into [`CallServerType`].
impl From<crate::types::CallServerTypeWebrtc> for CallServerType {
    fn from(val: crate::types::CallServerTypeWebrtc) -> Self {
        Self::Webrtc(Box::new(val))
    }
}

/// TDLib `CallServer` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallServer {
    /// Describes a server for relaying call data
    #[serde(rename(serialize = "callServer", deserialize = "callServer"))]
    CallServer(Box<crate::types::CallServer>),
}

impl CallServer {
    /// Convenience constructor to create a [`CallServer::CallServer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn call_server(val: crate::types::CallServer) -> Self {
        Self::CallServer(Box::new(val))
    }

}

/// Converts a [`crate::types::CallServer`] into [`CallServer`].
impl From<crate::types::CallServer> for CallServer {
    fn from(val: crate::types::CallServer) -> Self {
        Self::CallServer(Box::new(val))
    }
}

/// TDLib `CallId` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallId {
    /// Contains the call identifier
    #[serde(rename(serialize = "callId", deserialize = "callId"))]
    CallId(Box<crate::types::CallId>),
}

impl CallId {
    /// Convenience constructor to create a [`CallId::CallId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn call_id(val: crate::types::CallId) -> Self {
        Self::CallId(Box::new(val))
    }

}

/// Converts a [`crate::types::CallId`] into [`CallId`].
impl From<crate::types::CallId> for CallId {
    fn from(val: crate::types::CallId) -> Self {
        Self::CallId(Box::new(val))
    }
}

/// TDLib `GroupCallId` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallId {
    /// Contains the group call identifier
    #[serde(rename(serialize = "groupCallId", deserialize = "groupCallId"))]
    GroupCallId(Box<crate::types::GroupCallId>),
}

impl GroupCallId {
    /// Convenience constructor to create a [`GroupCallId::GroupCallId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_id(val: crate::types::GroupCallId) -> Self {
        Self::GroupCallId(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallId`] into [`GroupCallId`].
impl From<crate::types::GroupCallId> for GroupCallId {
    fn from(val: crate::types::GroupCallId) -> Self {
        Self::GroupCallId(Box::new(val))
    }
}

/// Describes a call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputCall {
    /// A just ended call
    #[serde(rename(serialize = "inputCallDiscarded", deserialize = "inputCallDiscarded"))]
    Discarded(Box<crate::types::InputCallDiscarded>),
    /// A call from a message of the type messageCall with non-zero messageCall.unique_id
    #[serde(rename(serialize = "inputCallFromMessage", deserialize = "inputCallFromMessage"))]
    FromMessage(Box<crate::types::InputCallFromMessage>),
}

impl InputCall {
    /// Convenience constructor to create a [`InputCall::Discarded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn discarded(val: crate::types::InputCallDiscarded) -> Self {
        Self::Discarded(Box::new(val))
    }

    /// Convenience constructor to create a [`InputCall::FromMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn from_message(val: crate::types::InputCallFromMessage) -> Self {
        Self::FromMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::InputCallDiscarded`] into [`InputCall`].
impl From<crate::types::InputCallDiscarded> for InputCall {
    fn from(val: crate::types::InputCallDiscarded) -> Self {
        Self::Discarded(Box::new(val))
    }
}

/// Converts a [`crate::types::InputCallFromMessage`] into [`InputCall`].
impl From<crate::types::InputCallFromMessage> for InputCall {
    fn from(val: crate::types::InputCallFromMessage) -> Self {
        Self::FromMessage(Box::new(val))
    }
}

/// Describes the current call state
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallState {
    /// The call is pending, waiting to be accepted by a user
    #[serde(rename(serialize = "callStatePending", deserialize = "callStatePending"))]
    Pending(Box<crate::types::CallStatePending>),
    /// The call has been answered and encryption keys are being exchanged
    #[serde(rename(serialize = "callStateExchangingKeys", deserialize = "callStateExchangingKeys"))]
    ExchangingKeys,
    /// The call is ready to use
    #[serde(rename(serialize = "callStateReady", deserialize = "callStateReady"))]
    Ready(Box<crate::types::CallStateReady>),
    /// The call is hanging up after discardCall has been called
    #[serde(rename(serialize = "callStateHangingUp", deserialize = "callStateHangingUp"))]
    HangingUp,
    /// The call has ended successfully
    #[serde(rename(serialize = "callStateDiscarded", deserialize = "callStateDiscarded"))]
    Discarded(Box<crate::types::CallStateDiscarded>),
    /// The call has ended with an error
    #[serde(rename(serialize = "callStateError", deserialize = "callStateError"))]
    Error(Box<crate::types::CallStateError>),
}

impl CallState {
    /// Convenience constructor to create a [`CallState::Pending`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pending(val: crate::types::CallStatePending) -> Self {
        Self::Pending(Box::new(val))
    }

    /// Convenience constructor to create a [`CallState::Ready`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ready(val: crate::types::CallStateReady) -> Self {
        Self::Ready(Box::new(val))
    }

    /// Convenience constructor to create a [`CallState::Discarded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn discarded(val: crate::types::CallStateDiscarded) -> Self {
        Self::Discarded(Box::new(val))
    }

    /// Convenience constructor to create a [`CallState::Error`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn error(val: crate::types::CallStateError) -> Self {
        Self::Error(Box::new(val))
    }

}

/// Converts a [`crate::types::CallStatePending`] into [`CallState`].
impl From<crate::types::CallStatePending> for CallState {
    fn from(val: crate::types::CallStatePending) -> Self {
        Self::Pending(Box::new(val))
    }
}

/// Converts a [`crate::types::CallStateReady`] into [`CallState`].
impl From<crate::types::CallStateReady> for CallState {
    fn from(val: crate::types::CallStateReady) -> Self {
        Self::Ready(Box::new(val))
    }
}

/// Converts a [`crate::types::CallStateDiscarded`] into [`CallState`].
impl From<crate::types::CallStateDiscarded> for CallState {
    fn from(val: crate::types::CallStateDiscarded) -> Self {
        Self::Discarded(Box::new(val))
    }
}

/// Converts a [`crate::types::CallStateError`] into [`CallState`].
impl From<crate::types::CallStateError> for CallState {
    fn from(val: crate::types::CallStateError) -> Self {
        Self::Error(Box::new(val))
    }
}

/// TDLib `GroupCallJoinParameters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallJoinParameters {
    /// Describes parameters used to join a group call
    #[serde(rename(serialize = "groupCallJoinParameters", deserialize = "groupCallJoinParameters"))]
    GroupCallJoinParameters(Box<crate::types::GroupCallJoinParameters>),
}

impl GroupCallJoinParameters {
    /// Convenience constructor to create a [`GroupCallJoinParameters::GroupCallJoinParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_join_parameters(val: crate::types::GroupCallJoinParameters) -> Self {
        Self::GroupCallJoinParameters(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallJoinParameters`] into [`GroupCallJoinParameters`].
impl From<crate::types::GroupCallJoinParameters> for GroupCallJoinParameters {
    fn from(val: crate::types::GroupCallJoinParameters) -> Self {
        Self::GroupCallJoinParameters(Box::new(val))
    }
}

/// Describes the quality of a group call video
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallVideoQuality {
    /// The worst available video quality
    #[serde(rename(serialize = "groupCallVideoQualityThumbnail", deserialize = "groupCallVideoQualityThumbnail"))]
    Thumbnail,
    /// The medium video quality
    #[serde(rename(serialize = "groupCallVideoQualityMedium", deserialize = "groupCallVideoQualityMedium"))]
    Medium,
    /// The best available video quality
    #[serde(rename(serialize = "groupCallVideoQualityFull", deserialize = "groupCallVideoQualityFull"))]
    Full,
}

/// TDLib `GroupCallStream` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallStream {
    /// Describes an available stream in a video chat or a live story
    #[serde(rename(serialize = "groupCallStream", deserialize = "groupCallStream"))]
    GroupCallStream(Box<crate::types::GroupCallStream>),
}

impl GroupCallStream {
    /// Convenience constructor to create a [`GroupCallStream::GroupCallStream`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_stream(val: crate::types::GroupCallStream) -> Self {
        Self::GroupCallStream(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallStream`] into [`GroupCallStream`].
impl From<crate::types::GroupCallStream> for GroupCallStream {
    fn from(val: crate::types::GroupCallStream) -> Self {
        Self::GroupCallStream(Box::new(val))
    }
}

/// TDLib `GroupCallStreams` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallStreams {
    /// Represents a list of group call streams
    #[serde(rename(serialize = "groupCallStreams", deserialize = "groupCallStreams"))]
    GroupCallStreams(Box<crate::types::GroupCallStreams>),
}

impl GroupCallStreams {
    /// Convenience constructor to create a [`GroupCallStreams::GroupCallStreams`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_streams(val: crate::types::GroupCallStreams) -> Self {
        Self::GroupCallStreams(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallStreams`] into [`GroupCallStreams`].
impl From<crate::types::GroupCallStreams> for GroupCallStreams {
    fn from(val: crate::types::GroupCallStreams) -> Self {
        Self::GroupCallStreams(Box::new(val))
    }
}

/// TDLib `GroupCallRecentSpeaker` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallRecentSpeaker {
    /// Describes a recently speaking participant in a group call
    #[serde(rename(serialize = "groupCallRecentSpeaker", deserialize = "groupCallRecentSpeaker"))]
    GroupCallRecentSpeaker(Box<crate::types::GroupCallRecentSpeaker>),
}

impl GroupCallRecentSpeaker {
    /// Convenience constructor to create a [`GroupCallRecentSpeaker::GroupCallRecentSpeaker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_recent_speaker(val: crate::types::GroupCallRecentSpeaker) -> Self {
        Self::GroupCallRecentSpeaker(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallRecentSpeaker`] into [`GroupCallRecentSpeaker`].
impl From<crate::types::GroupCallRecentSpeaker> for GroupCallRecentSpeaker {
    fn from(val: crate::types::GroupCallRecentSpeaker) -> Self {
        Self::GroupCallRecentSpeaker(Box::new(val))
    }
}

/// TDLib `GroupCall` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCall {
    /// Describes a group call
    #[serde(rename(serialize = "groupCall", deserialize = "groupCall"))]
    GroupCall(Box<crate::types::GroupCall>),
}

impl GroupCall {
    /// Convenience constructor to create a [`GroupCall::GroupCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call(val: crate::types::GroupCall) -> Self {
        Self::GroupCall(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCall`] into [`GroupCall`].
impl From<crate::types::GroupCall> for GroupCall {
    fn from(val: crate::types::GroupCall) -> Self {
        Self::GroupCall(Box::new(val))
    }
}

/// TDLib `GroupCallVideoSourceGroup` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallVideoSourceGroup {
    /// Describes a group of video synchronization source identifiers
    #[serde(rename(serialize = "groupCallVideoSourceGroup", deserialize = "groupCallVideoSourceGroup"))]
    GroupCallVideoSourceGroup(Box<crate::types::GroupCallVideoSourceGroup>),
}

impl GroupCallVideoSourceGroup {
    /// Convenience constructor to create a [`GroupCallVideoSourceGroup::GroupCallVideoSourceGroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_video_source_group(val: crate::types::GroupCallVideoSourceGroup) -> Self {
        Self::GroupCallVideoSourceGroup(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallVideoSourceGroup`] into [`GroupCallVideoSourceGroup`].
impl From<crate::types::GroupCallVideoSourceGroup> for GroupCallVideoSourceGroup {
    fn from(val: crate::types::GroupCallVideoSourceGroup) -> Self {
        Self::GroupCallVideoSourceGroup(Box::new(val))
    }
}

/// TDLib `GroupCallParticipantVideoInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallParticipantVideoInfo {
    /// Contains information about a group call participant's video channel
    #[serde(rename(serialize = "groupCallParticipantVideoInfo", deserialize = "groupCallParticipantVideoInfo"))]
    GroupCallParticipantVideoInfo(Box<crate::types::GroupCallParticipantVideoInfo>),
}

impl GroupCallParticipantVideoInfo {
    /// Convenience constructor to create a [`GroupCallParticipantVideoInfo::GroupCallParticipantVideoInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_participant_video_info(val: crate::types::GroupCallParticipantVideoInfo) -> Self {
        Self::GroupCallParticipantVideoInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallParticipantVideoInfo`] into [`GroupCallParticipantVideoInfo`].
impl From<crate::types::GroupCallParticipantVideoInfo> for GroupCallParticipantVideoInfo {
    fn from(val: crate::types::GroupCallParticipantVideoInfo) -> Self {
        Self::GroupCallParticipantVideoInfo(Box::new(val))
    }
}

/// TDLib `GroupCallParticipant` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallParticipant {
    /// Represents a group call participant
    #[serde(rename(serialize = "groupCallParticipant", deserialize = "groupCallParticipant"))]
    GroupCallParticipant(Box<crate::types::GroupCallParticipant>),
}

impl GroupCallParticipant {
    /// Convenience constructor to create a [`GroupCallParticipant::GroupCallParticipant`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_participant(val: crate::types::GroupCallParticipant) -> Self {
        Self::GroupCallParticipant(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallParticipant`] into [`GroupCallParticipant`].
impl From<crate::types::GroupCallParticipant> for GroupCallParticipant {
    fn from(val: crate::types::GroupCallParticipant) -> Self {
        Self::GroupCallParticipant(Box::new(val))
    }
}

/// TDLib `GroupCallParticipants` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallParticipants {
    /// Contains identifiers of group call participants
    #[serde(rename(serialize = "groupCallParticipants", deserialize = "groupCallParticipants"))]
    GroupCallParticipants(Box<crate::types::GroupCallParticipants>),
}

impl GroupCallParticipants {
    /// Convenience constructor to create a [`GroupCallParticipants::GroupCallParticipants`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_participants(val: crate::types::GroupCallParticipants) -> Self {
        Self::GroupCallParticipants(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallParticipants`] into [`GroupCallParticipants`].
impl From<crate::types::GroupCallParticipants> for GroupCallParticipants {
    fn from(val: crate::types::GroupCallParticipants) -> Self {
        Self::GroupCallParticipants(Box::new(val))
    }
}

/// TDLib `GroupCallInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallInfo {
    /// Contains information about a just created or just joined group call
    #[serde(rename(serialize = "groupCallInfo", deserialize = "groupCallInfo"))]
    GroupCallInfo(Box<crate::types::GroupCallInfo>),
}

impl GroupCallInfo {
    /// Convenience constructor to create a [`GroupCallInfo::GroupCallInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_info(val: crate::types::GroupCallInfo) -> Self {
        Self::GroupCallInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallInfo`] into [`GroupCallInfo`].
impl From<crate::types::GroupCallInfo> for GroupCallInfo {
    fn from(val: crate::types::GroupCallInfo) -> Self {
        Self::GroupCallInfo(Box::new(val))
    }
}

/// TDLib `GroupCallMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallMessage {
    /// Represents a message sent in a group call
    #[serde(rename(serialize = "groupCallMessage", deserialize = "groupCallMessage"))]
    GroupCallMessage(Box<crate::types::GroupCallMessage>),
}

impl GroupCallMessage {
    /// Convenience constructor to create a [`GroupCallMessage::GroupCallMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_message(val: crate::types::GroupCallMessage) -> Self {
        Self::GroupCallMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallMessage`] into [`GroupCallMessage`].
impl From<crate::types::GroupCallMessage> for GroupCallMessage {
    fn from(val: crate::types::GroupCallMessage) -> Self {
        Self::GroupCallMessage(Box::new(val))
    }
}

/// TDLib `GroupCallMessageLevel` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallMessageLevel {
    /// Represents a level of features for a message sent in a live story group call
    #[serde(rename(serialize = "groupCallMessageLevel", deserialize = "groupCallMessageLevel"))]
    GroupCallMessageLevel(Box<crate::types::GroupCallMessageLevel>),
}

impl GroupCallMessageLevel {
    /// Convenience constructor to create a [`GroupCallMessageLevel::GroupCallMessageLevel`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_message_level(val: crate::types::GroupCallMessageLevel) -> Self {
        Self::GroupCallMessageLevel(Box::new(val))
    }

}

/// Converts a [`crate::types::GroupCallMessageLevel`] into [`GroupCallMessageLevel`].
impl From<crate::types::GroupCallMessageLevel> for GroupCallMessageLevel {
    fn from(val: crate::types::GroupCallMessageLevel) -> Self {
        Self::GroupCallMessageLevel(Box::new(val))
    }
}

/// Describes result of group call participant invitation
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InviteGroupCallParticipantResult {
    /// The user can't be invited due to their privacy settings
    #[serde(rename(serialize = "inviteGroupCallParticipantResultUserPrivacyRestricted", deserialize = "inviteGroupCallParticipantResultUserPrivacyRestricted"))]
    UserPrivacyRestricted,
    /// The user can't be invited because they are already a participant of the call
    #[serde(rename(serialize = "inviteGroupCallParticipantResultUserAlreadyParticipant", deserialize = "inviteGroupCallParticipantResultUserAlreadyParticipant"))]
    UserAlreadyParticipant,
    /// The user can't be invited because they were banned by the owner of the call and can be invited back only by the owner of the group call
    #[serde(rename(serialize = "inviteGroupCallParticipantResultUserWasBanned", deserialize = "inviteGroupCallParticipantResultUserWasBanned"))]
    UserWasBanned,
    /// The user was invited and a service message of the type messageGroupCall was sent which can be used in declineGroupCallInvitation to cancel the invitation
    #[serde(rename(serialize = "inviteGroupCallParticipantResultSuccess", deserialize = "inviteGroupCallParticipantResultSuccess"))]
    Success(Box<crate::types::InviteGroupCallParticipantResultSuccess>),
}

impl InviteGroupCallParticipantResult {
    /// Convenience constructor to create a [`InviteGroupCallParticipantResult::Success`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn success(val: crate::types::InviteGroupCallParticipantResultSuccess) -> Self {
        Self::Success(Box::new(val))
    }

}

/// Converts a [`crate::types::InviteGroupCallParticipantResultSuccess`] into [`InviteGroupCallParticipantResult`].
impl From<crate::types::InviteGroupCallParticipantResultSuccess> for InviteGroupCallParticipantResult {
    fn from(val: crate::types::InviteGroupCallParticipantResultSuccess) -> Self {
        Self::Success(Box::new(val))
    }
}

/// Describes data channel for a group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallDataChannel {
    /// The main data channel for audio and video data
    #[serde(rename(serialize = "groupCallDataChannelMain", deserialize = "groupCallDataChannelMain"))]
    Main,
    /// The data channel for screen sharing
    #[serde(rename(serialize = "groupCallDataChannelScreenSharing", deserialize = "groupCallDataChannelScreenSharing"))]
    ScreenSharing,
}

/// Describes a non-joined group call that isn't bound to a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputGroupCall {
    /// The group call is accessible through a link
    #[serde(rename(serialize = "inputGroupCallLink", deserialize = "inputGroupCallLink"))]
    Link(Box<crate::types::InputGroupCallLink>),
    /// The group call is accessible through a message of the type messageGroupCall
    #[serde(rename(serialize = "inputGroupCallMessage", deserialize = "inputGroupCallMessage"))]
    Message(Box<crate::types::InputGroupCallMessage>),
}

impl InputGroupCall {
    /// Convenience constructor to create a [`InputGroupCall::Link`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn link(val: crate::types::InputGroupCallLink) -> Self {
        Self::Link(Box::new(val))
    }

    /// Convenience constructor to create a [`InputGroupCall::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::InputGroupCallMessage) -> Self {
        Self::Message(Box::new(val))
    }

}

/// Converts a [`crate::types::InputGroupCallLink`] into [`InputGroupCall`].
impl From<crate::types::InputGroupCallLink> for InputGroupCall {
    fn from(val: crate::types::InputGroupCallLink) -> Self {
        Self::Link(Box::new(val))
    }
}

/// Converts a [`crate::types::InputGroupCallMessage`] into [`InputGroupCall`].
impl From<crate::types::InputGroupCallMessage> for InputGroupCall {
    fn from(val: crate::types::InputGroupCallMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Describes the exact type of problem with a call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallProblem {
    /// The user heard their own voice
    #[serde(rename(serialize = "callProblemEcho", deserialize = "callProblemEcho"))]
    Echo,
    /// The user heard background noise
    #[serde(rename(serialize = "callProblemNoise", deserialize = "callProblemNoise"))]
    Noise,
    /// The other side kept disappearing
    #[serde(rename(serialize = "callProblemInterruptions", deserialize = "callProblemInterruptions"))]
    Interruptions,
    /// The speech was distorted
    #[serde(rename(serialize = "callProblemDistortedSpeech", deserialize = "callProblemDistortedSpeech"))]
    DistortedSpeech,
    /// The user couldn't hear the other side
    #[serde(rename(serialize = "callProblemSilentLocal", deserialize = "callProblemSilentLocal"))]
    SilentLocal,
    /// The other side couldn't hear the user
    #[serde(rename(serialize = "callProblemSilentRemote", deserialize = "callProblemSilentRemote"))]
    SilentRemote,
    /// The call ended unexpectedly
    #[serde(rename(serialize = "callProblemDropped", deserialize = "callProblemDropped"))]
    Dropped,
    /// The video was distorted
    #[serde(rename(serialize = "callProblemDistortedVideo", deserialize = "callProblemDistortedVideo"))]
    DistortedVideo,
    /// The video was pixelated
    #[serde(rename(serialize = "callProblemPixelatedVideo", deserialize = "callProblemPixelatedVideo"))]
    PixelatedVideo,
}

/// TDLib `Call` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Call {
    /// Describes a call
    #[serde(rename(serialize = "call", deserialize = "call"))]
    Call(Box<crate::types::Call>),
}

impl Call {
    /// Convenience constructor to create a [`Call::Call`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn call(val: crate::types::Call) -> Self {
        Self::Call(Box::new(val))
    }

}

/// Converts a [`crate::types::Call`] into [`Call`].
impl From<crate::types::Call> for Call {
    fn from(val: crate::types::Call) -> Self {
        Self::Call(Box::new(val))
    }
}

/// Represents a payload of a callback query
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallbackQueryPayload {
    /// The payload for a general callback button
    #[serde(rename(serialize = "callbackQueryPayloadData", deserialize = "callbackQueryPayloadData"))]
    Data(Box<crate::types::CallbackQueryPayloadData>),
    /// The payload for a callback button requiring password
    #[serde(rename(serialize = "callbackQueryPayloadDataWithPassword", deserialize = "callbackQueryPayloadDataWithPassword"))]
    DataWithPassword(Box<crate::types::CallbackQueryPayloadDataWithPassword>),
    /// The payload for a game callback button
    #[serde(rename(serialize = "callbackQueryPayloadGame", deserialize = "callbackQueryPayloadGame"))]
    Game(Box<crate::types::CallbackQueryPayloadGame>),
}

impl CallbackQueryPayload {
    /// Convenience constructor to create a [`CallbackQueryPayload::Data`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn data(val: crate::types::CallbackQueryPayloadData) -> Self {
        Self::Data(Box::new(val))
    }

    /// Convenience constructor to create a [`CallbackQueryPayload::DataWithPassword`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn data_with_password(val: crate::types::CallbackQueryPayloadDataWithPassword) -> Self {
        Self::DataWithPassword(Box::new(val))
    }

    /// Convenience constructor to create a [`CallbackQueryPayload::Game`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game(val: crate::types::CallbackQueryPayloadGame) -> Self {
        Self::Game(Box::new(val))
    }

}

/// Converts a [`crate::types::CallbackQueryPayloadData`] into [`CallbackQueryPayload`].
impl From<crate::types::CallbackQueryPayloadData> for CallbackQueryPayload {
    fn from(val: crate::types::CallbackQueryPayloadData) -> Self {
        Self::Data(Box::new(val))
    }
}

/// Converts a [`crate::types::CallbackQueryPayloadDataWithPassword`] into [`CallbackQueryPayload`].
impl From<crate::types::CallbackQueryPayloadDataWithPassword> for CallbackQueryPayload {
    fn from(val: crate::types::CallbackQueryPayloadDataWithPassword) -> Self {
        Self::DataWithPassword(Box::new(val))
    }
}

/// Converts a [`crate::types::CallbackQueryPayloadGame`] into [`CallbackQueryPayload`].
impl From<crate::types::CallbackQueryPayloadGame> for CallbackQueryPayload {
    fn from(val: crate::types::CallbackQueryPayloadGame) -> Self {
        Self::Game(Box::new(val))
    }
}

/// TDLib `CallbackQueryAnswer` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallbackQueryAnswer {
    /// Contains a bot's answer to a callback query
    #[serde(rename(serialize = "callbackQueryAnswer", deserialize = "callbackQueryAnswer"))]
    CallbackQueryAnswer(Box<crate::types::CallbackQueryAnswer>),
}

impl CallbackQueryAnswer {
    /// Convenience constructor to create a [`CallbackQueryAnswer::CallbackQueryAnswer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn callback_query_answer(val: crate::types::CallbackQueryAnswer) -> Self {
        Self::CallbackQueryAnswer(Box::new(val))
    }

}

/// Converts a [`crate::types::CallbackQueryAnswer`] into [`CallbackQueryAnswer`].
impl From<crate::types::CallbackQueryAnswer> for CallbackQueryAnswer {
    fn from(val: crate::types::CallbackQueryAnswer) -> Self {
        Self::CallbackQueryAnswer(Box::new(val))
    }
}


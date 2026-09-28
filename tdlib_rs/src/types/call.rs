//!
//! TDLib `call` domain types.
//!
//! Types, enums, and functions for 1-on-1 calls, group calls, and live video chats.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A digit-only authentication code is delivered via a phone call to the specified phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeCall {
    /// Length of the code
    pub length: i32,
}

/// An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeFlashCall {
    /// Pattern of the phone number from which the call will be made
    pub pattern: String,
}

/// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeMissedCall {
    /// Prefix of the phone number from which the call will be made
    pub phone_number_prefix: String,
    /// Number of digits in the code, excluding the prefix
    pub length: i32,
}

/// The transaction is a sending of a paid group call message; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidGroupCallMessageSend {
    /// Identifier of the chat that received the payment
    pub chat_id: i64,
}

/// The transaction is a receiving of a paid group call message; relevant for regular users and channel chats only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidGroupCallMessageReceive {
    /// Identifier of the sender of the message
    pub sender_id: crate::enums::MessageSender,
    /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars paid for message sending
    pub commission_per_mille: i32,
    /// The Telegram Star amount that was received by Telegram; can be negative for refunds
    pub commission_star_amount: crate::types::StarAmount,
}

/// The transaction is a sending of a paid group reaction; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidGroupCallReactionSend {
    /// Identifier of the chat that received the payment
    pub chat_id: i64,
}

/// The transaction is a receiving of a paid group call reaction; relevant for regular users and channel chats only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidGroupCallReactionReceive {
    /// Identifier of the sender of the reaction
    pub sender_id: crate::enums::MessageSender,
    /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars paid for reaction sending
    pub commission_per_mille: i32,
    /// The Telegram Star amount that was received by Telegram; can be negative for refunds
    pub commission_star_amount: crate::types::StarAmount,
}

/// A button that sends a callback query to a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeCallback {
    /// Data to be sent to the bot via a callback query
    pub data: String,
}

/// A button that asks for the 2-step verification password of the current user and then sends a callback query to a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeCallbackWithPassword {
    /// Data to be sent to the bot via a callback query
    pub data: String,
}

/// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeCallbackGame {
}

/// The link is a link to a group call that isn't bound to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeGroupCall {
}

/// A message with information about an ended call
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageCall {
    /// Persistent unique call identifier; 0 for calls from other devices, which can't be passed as inputCallFromMessage
    #[serde_as(as = "DisplayFromStr")]
    pub unique_id: i64,
    /// True, if the call was a video call
    pub is_video: bool,
    /// Reason why the call was discarded
    pub discard_reason: crate::enums::CallDiscardReason,
    /// Call duration, in seconds
    pub duration: i32,
}

/// A message with information about a group call not bound to a chat. If the message is incoming, the call isn't active, isn't missed, and has no duration,
/// and getOption("can_accept_calls") is true, then incoming call screen must be shown to the user. Use getGroupCallParticipants to show current group call participants on the screen.
/// Use joinGroupCall to accept the call or declineGroupCallInvitation to decline it. If the call become active or missed, then the call screen must be hidden
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageGroupCall {
    /// Persistent unique group call identifier
    #[serde_as(as = "DisplayFromStr")]
    pub unique_id: i64,
    /// True, if the call is active, i.e. the called user joined the call
    pub is_active: bool,
    /// True, if the called user missed or declined the call
    pub was_missed: bool,
    /// True, if the call is a video call
    pub is_video: bool,
    /// Call duration, in seconds; for left calls only
    pub duration: i32,
    /// Identifiers of some other call participants
    pub other_participant_ids: Vec<crate::enums::MessageSender>,
}

/// The call wasn't discarded, or the reason is unknown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallDiscardReasonEmpty {
}

/// The call was ended before the conversation started. It was canceled by the caller or missed by the other party
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallDiscardReasonMissed {
}

/// The call was ended before the conversation started. It was declined by the other party
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallDiscardReasonDeclined {
}

/// The call was ended during the conversation because the users were disconnected
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallDiscardReasonDisconnected {
}

/// The call was ended because one of the parties hung up
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallDiscardReasonHungUp {
}

/// The call was ended because it has been upgraded to a group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallDiscardReasonUpgradeToGroupCall {
    /// Invite link for the group call
    pub invite_link: String,
}

/// Specifies the supported call protocols
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProtocol {
    /// True, if UDP peer-to-peer connections are supported
    pub udp_p2p: bool,
    /// True, if connection through UDP reflectors is supported
    pub udp_reflector: bool,
    /// The minimum supported API layer; use 65
    pub min_layer: i32,
    /// The maximum supported API layer; use 92
    pub max_layer: i32,
    /// List of supported tgcalls versions
    pub library_versions: Vec<String>,
}

/// A Telegram call reflector
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallServerTypeTelegramReflector {
    /// A peer tag to be used with the reflector
    pub peer_tag: String,
    /// True, if the server uses TCP instead of UDP
    pub is_tcp: bool,
}

/// A WebRTC server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallServerTypeWebrtc {
    /// Username to be used for authentication
    pub username: String,
    /// Authentication password
    pub password: String,
    /// True, if the server supports TURN
    pub supports_turn: bool,
    /// True, if the server supports STUN
    pub supports_stun: bool,
}

/// Describes a server for relaying call data
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CallServer {
    /// Server identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Server IPv4 address
    pub ip_address: String,
    /// Server IPv6 address
    pub ipv6_address: String,
    /// Server port number
    pub port: i32,
    /// Server type
    #[serde(rename = "type")]
    pub r#type: crate::enums::CallServerType,
}

/// Contains the call identifier
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallId {
    /// Call identifier
    pub id: i32,
}

/// Contains the group call identifier
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallId {
    /// Group call identifier
    pub id: i32,
}

/// A just ended call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCallDiscarded {
    /// Identifier of the call
    pub call_id: i32,
}

/// A call from a message of the type messageCall with non-zero messageCall.unique_id
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCallFromMessage {
    /// Chat identifier of the message
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
}

/// The call is pending, waiting to be accepted by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallStatePending {
    /// True, if the call has already been created by the server
    pub is_created: bool,
    /// True, if the call has already been received by the other party
    pub is_received: bool,
}

/// The call has been answered and encryption keys are being exchanged
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallStateExchangingKeys {
}

/// The call is ready to use
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallStateReady {
    /// Call protocols supported by the other call participant
    pub protocol: crate::types::CallProtocol,
    /// List of available call servers
    pub servers: Vec<crate::types::CallServer>,
    /// A JSON-encoded call config
    pub config: String,
    /// Call encryption key
    pub encryption_key: String,
    /// Encryption key fingerprint represented as 4 emoji
    pub emojis: Vec<String>,
    /// True, if peer-to-peer connection is allowed by users privacy settings
    pub allow_p2p: bool,
    /// True, if the other party supports upgrading of the call to a group call
    pub is_group_call_supported: bool,
    /// Custom JSON-encoded call parameters to be passed to tgcalls
    pub custom_parameters: String,
}

/// The call is hanging up after discardCall has been called
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallStateHangingUp {
}

/// The call has ended successfully
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CallStateDiscarded {
    /// The reason why the call has ended
    pub reason: crate::enums::CallDiscardReason,
    /// True, if the call rating must be sent to the server
    pub need_rating: bool,
    /// True, if the call debug information must be sent to the server
    pub need_debug_information: bool,
    /// True, if the call log must be sent to the server
    pub need_log: bool,
}

/// The call has ended with an error
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallStateError {
    /// Error. An error with the code 4005000 will be returned if an outgoing call is missed because of an expired timeout
    pub error: crate::types::Error,
}

/// Describes parameters used to join a group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallJoinParameters {
    /// Audio channel synchronization source identifier; received from tgcalls
    pub audio_source_id: i32,
    /// Group call join payload; received from tgcalls
    pub payload: String,
    /// Pass true to join the call with muted microphone
    pub is_muted: bool,
    /// Pass true if the user's video is enabled
    pub is_my_video_enabled: bool,
}

/// The worst available video quality
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallVideoQualityThumbnail {
}

/// The medium video quality
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallVideoQualityMedium {
}

/// The best available video quality
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallVideoQualityFull {
}

/// Describes an available stream in a video chat or a live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallStream {
    /// Identifier of an audio/video channel
    pub channel_id: i32,
    /// Scale of segment durations in the stream. The duration is 1000/(2**scale) milliseconds
    pub scale: i32,
    /// Point in time when the stream currently ends; Unix timestamp in milliseconds
    pub time_offset: i64,
}

/// Represents a list of group call streams
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallStreams {
    /// A list of group call streams
    pub streams: Vec<crate::types::GroupCallStream>,
}

/// Describes a recently speaking participant in a group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GroupCallRecentSpeaker {
    /// Group call participant identifier
    pub participant_id: crate::enums::MessageSender,
    /// True, is the user has spoken recently
    pub is_speaking: bool,
}

/// Describes a group call
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCall {
    /// Group call identifier
    pub id: i32,
    /// Persistent unique group call identifier
    #[serde_as(as = "DisplayFromStr")]
    pub unique_id: i64,
    /// Group call title; for video chats only
    pub title: String,
    /// Invite link for the group call; for group calls that aren't bound to a chat. For video chats call getVideoChatInviteLink to get the link.
    /// For live stories in chats with username call getInternalLink with internalLinkTypeLiveStory
    pub invite_link: String,
    /// The minimum number of Telegram Stars that must be paid by general participant for each sent message to the call; for live stories only
    pub paid_message_star_count: i64,
    /// Point in time (Unix timestamp) when the group call is expected to be started by an administrator; 0 if it is already active or was ended; for video chats only
    pub scheduled_start_date: i32,
    /// True, if the group call is scheduled and the current user will receive a notification when the group call starts; for video chats only
    pub enabled_start_notification: bool,
    /// True, if the call is active
    pub is_active: bool,
    /// True, if the call is bound to a chat
    pub is_video_chat: bool,
    /// True, if the call is a live story of a chat
    pub is_live_story: bool,
    /// True, if the call is an RTMP stream instead of an ordinary video chat; for video chats and live stories only
    pub is_rtmp_stream: bool,
    /// True, if the call is joined
    pub is_joined: bool,
    /// True, if user was kicked from the call because of network loss and the call needs to be rejoined
    pub need_rejoin: bool,
    /// True, if the user is the owner of the call and can end the call, change volume level of other users, or ban users there; for group calls that aren't bound to a chat
    pub is_owned: bool,
    /// True, if the current user can manage the group call; for video chats and live stories only
    pub can_be_managed: bool,
    /// Number of participants in the group call
    pub participant_count: i32,
    /// True, if group call participants, which are muted, aren't returned in participant list; for video chats only
    pub has_hidden_listeners: bool,
    /// True, if all group call participants are loaded
    pub loaded_all_participants: bool,
    /// Message sender chosen to send messages to the group call; for live stories only; may be null if the call isn't a live story
    pub message_sender_id: Option<crate::enums::MessageSender>,
    /// At most 3 recently speaking users in the group call
    pub recent_speakers: Vec<crate::types::GroupCallRecentSpeaker>,
    /// True, if the current user's video is enabled
    pub is_my_video_enabled: bool,
    /// True, if the current user's video is paused
    pub is_my_video_paused: bool,
    /// True, if the current user can broadcast video or share screen
    pub can_enable_video: bool,
    /// True, if only group call administrators can unmute new participants; for video chats only
    pub mute_new_participants: bool,
    /// True, if the current user can enable or disable mute_new_participants setting; for video chats only
    pub can_toggle_mute_new_participants: bool,
    /// True, if the current user can send messages to the group call
    pub can_send_messages: bool,
    /// True, if sending of messages is allowed in the group call
    pub are_messages_allowed: bool,
    /// True, if the current user can enable or disable sending of messages in the group call
    pub can_toggle_are_messages_allowed: bool,
    /// True, if the user can delete messages in the group call
    pub can_delete_messages: bool,
    /// Duration of the ongoing group call recording, in seconds; 0 if none. An updateGroupCall update is not triggered when value of this field changes, but the same recording goes on
    pub record_duration: i32,
    /// True, if a video file is being recorded for the call
    pub is_video_recorded: bool,
    /// Call duration, in seconds; for ended calls only
    pub duration: i32,
}

/// Describes a group of video synchronization source identifiers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallVideoSourceGroup {
    /// The semantics of sources, one of "SIM" or "FID"
    pub semantics: String,
    /// The list of synchronization source identifiers
    pub source_ids: Vec<i32>,
}

/// Contains information about a group call participant's video channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallParticipantVideoInfo {
    /// List of synchronization source groups of the video
    pub source_groups: Vec<crate::types::GroupCallVideoSourceGroup>,
    /// Video channel endpoint identifier
    pub endpoint_id: String,
    /// True, if the video is paused. This flag needs to be ignored, if new video frames are received
    pub is_paused: bool,
}

/// Represents a group call participant
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GroupCallParticipant {
    /// Identifier of the group call participant
    pub participant_id: crate::enums::MessageSender,
    /// User's audio channel synchronization source identifier
    pub audio_source_id: i32,
    /// User's screen sharing audio channel synchronization source identifier
    pub screen_sharing_audio_source_id: i32,
    /// Information about user's video channel; may be null if there is no active video
    pub video_info: Option<crate::types::GroupCallParticipantVideoInfo>,
    /// Information about user's screen sharing video channel; may be null if there is no active screen sharing video
    pub screen_sharing_video_info: Option<crate::types::GroupCallParticipantVideoInfo>,
    /// The participant user's bio or the participant chat's description
    pub bio: String,
    /// True, if the participant is the current user
    pub is_current_user: bool,
    /// True, if the participant is speaking as set by setGroupCallParticipantIsSpeaking
    pub is_speaking: bool,
    /// True, if the participant hand is raised
    pub is_hand_raised: bool,
    /// True, if the current user can mute the participant for all other group call participants
    pub can_be_muted_for_all_users: bool,
    /// True, if the current user can allow the participant to unmute themselves or unmute the participant (if the participant is the current user)
    pub can_be_unmuted_for_all_users: bool,
    /// True, if the current user can mute the participant only for self
    pub can_be_muted_for_current_user: bool,
    /// True, if the current user can unmute the participant for self
    pub can_be_unmuted_for_current_user: bool,
    /// True, if the participant is muted for all users
    pub is_muted_for_all_users: bool,
    /// True, if the participant is muted for the current user
    pub is_muted_for_current_user: bool,
    /// True, if the participant is muted for all users, but can unmute themselves
    pub can_unmute_self: bool,
    /// Participant's volume level; 1-20000 in hundreds of percents
    pub volume_level: i32,
    /// User's order in the group call participant list. Orders must be compared lexicographically. The bigger is order, the higher is user in the list. If order is empty, the user must be removed from the participant list
    pub order: String,
}

/// Contains identifiers of group call participants
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallParticipants {
    /// Total number of group call participants
    pub total_count: i32,
    /// Identifiers of the participants
    pub participant_ids: Vec<crate::enums::MessageSender>,
}

/// Contains information about a just created or just joined group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallInfo {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// Join response payload for tgcalls; empty if the call isn't joined
    pub join_payload: String,
}

/// Represents a message sent in a group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GroupCallMessage {
    /// Unique message identifier within the group call
    pub message_id: i32,
    /// Identifier of the sender of the message
    pub sender_id: crate::enums::MessageSender,
    /// Point in time (Unix timestamp) when the message was sent
    pub date: i32,
    /// Text of the message. If empty, then the message is a paid reaction in a live story
    pub text: crate::types::FormattedText,
    /// The number of Telegram Stars that were paid to send the message; for live stories only
    pub paid_message_star_count: i64,
    /// True, if the message is sent by the owner of the call and must be treated as a message of the maximum level; for live stories only
    pub is_from_owner: bool,
    /// True, if the message can be deleted by the current user; for live stories only
    pub can_be_deleted: bool,
}

/// Represents a level of features for a message sent in a live story group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallMessageLevel {
    /// The minimum number of Telegram Stars required to get features of the level
    pub min_star_count: i64,
    /// The amount of time the message of this level will be pinned, in seconds
    pub pin_duration: i32,
    /// The maximum allowed length of the message text
    pub max_text_length: i32,
    /// The maximum allowed number of custom emoji in the message text
    pub max_custom_emoji_count: i32,
    /// The first color used to show the message text in the RGB format
    pub first_color: i32,
    /// The second color used to show the message text in the RGB format
    pub second_color: i32,
    /// Background color for the message the RGB format
    pub background_color: i32,
}

/// The user can't be invited due to their privacy settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InviteGroupCallParticipantResultUserPrivacyRestricted {
}

/// The user can't be invited because they are already a participant of the call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InviteGroupCallParticipantResultUserAlreadyParticipant {
}

/// The user can't be invited because they were banned by the owner of the call and can be invited back only by the owner of the group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InviteGroupCallParticipantResultUserWasBanned {
}

/// The user was invited and a service message of the type messageGroupCall was sent which can be used in declineGroupCallInvitation to cancel the invitation
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InviteGroupCallParticipantResultSuccess {
    /// Identifier of the chat with the invitation message
    pub chat_id: i64,
    /// Identifier of the message
    pub message_id: i64,
}

/// The main data channel for audio and video data
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallDataChannelMain {
}

/// The data channel for screen sharing
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallDataChannelScreenSharing {
}

/// The group call is accessible through a link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputGroupCallLink {
    /// The link for the group call
    pub link: String,
}

/// The group call is accessible through a message of the type messageGroupCall
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputGroupCallMessage {
    /// Identifier of the chat with the message
    pub chat_id: i64,
    /// Identifier of the message of the type messageGroupCall
    pub message_id: i64,
}

/// The user heard their own voice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemEcho {
}

/// The user heard background noise
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemNoise {
}

/// The other side kept disappearing
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemInterruptions {
}

/// The speech was distorted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemDistortedSpeech {
}

/// The user couldn't hear the other side
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemSilentLocal {
}

/// The other side couldn't hear the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemSilentRemote {
}

/// The call ended unexpectedly
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemDropped {
}

/// The video was distorted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemDistortedVideo {
}

/// The video was pixelated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallProblemPixelatedVideo {
}

/// Describes a call
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Call {
    /// Call identifier, not persistent
    pub id: i32,
    /// Persistent unique call identifier; 0 if isn't assigned yet by the server
    #[serde_as(as = "DisplayFromStr")]
    pub unique_id: i64,
    /// User identifier of the other call participant
    pub user_id: i64,
    /// True, if the call is outgoing
    pub is_outgoing: bool,
    /// True, if the call is a video call
    pub is_video: bool,
    /// Call state
    pub state: crate::enums::CallState,
}

/// The payload for a general callback button
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallbackQueryPayloadData {
    /// Data that was attached to the callback button
    pub data: String,
}

/// The payload for a callback button requiring password
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallbackQueryPayloadDataWithPassword {
    /// The 2-step verification password for the current user
    pub password: String,
    /// Data that was attached to the callback button
    pub data: String,
}

/// The payload for a game callback button
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallbackQueryPayloadGame {
    /// A short name of the game that was attached to the callback button
    pub game_short_name: String,
}

/// Contains a bot's answer to a callback query
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallbackQueryAnswer {
    /// Text of the answer
    pub text: String,
    /// True, if an alert must be shown to the user instead of a toast notification
    pub show_alert: bool,
    /// URL to be opened
    pub url: String,
}

/// New call was received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationTypeNewCall {
    /// Call identifier
    pub call_id: i32,
}

/// A group containing notifications of type notificationTypeNewCall
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationGroupTypeCalls {
}

/// A privacy setting for managing whether the user can be called
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingAllowCalls {
}

/// A privacy setting for managing whether peer-to-peer connections can be used for calls
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingAllowPeerToPeerCalls {
}

/// The link is a link to the Call tab or page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeCallsPage {
    /// Section of the page; may be one of
    /// "", "all", "missed", "edit", "show-tab", "start-call"
    pub section: String,
}

/// The link is a link to a group call that isn't bound to a chat. Use getGroupCallParticipants to get the list of group call participants and show them on the join group call screen.
/// Call joinGroupCall with the given invite_link to join the call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeGroupCall {
    /// Internal representation of the invite link
    pub invite_link: String,
}

/// Contains information about the total amount of data that was used for calls
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NetworkStatisticsEntryCall {
    /// Type of the network the data was sent through. Call setNetworkType to maintain the actual network type
    pub network_type: crate::enums::NetworkType,
    /// Total number of bytes sent
    pub sent_bytes: i64,
    /// Total number of bytes received
    pub received_bytes: i64,
    /// Total call duration, in seconds
    pub duration: f64,
}

/// A category containing frequently used chats used for calls
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryCalls {
}

/// New call was created or information about a call was updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateCall {
    /// New data about a call
    pub call: crate::types::Call,
}

/// Information about a group call was updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCall {
    /// New data about the group call
    pub group_call: crate::types::GroupCall,
}

/// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallParticipant {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// New data about the participant
    pub participant: crate::types::GroupCallParticipant,
}

/// The list of group call participants that can send and receive encrypted call data has changed; for group calls not bound to a chat only
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallParticipants {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// New list of group call participant user identifiers. The identifiers may be invalid or the corresponding users may be unknown.
    /// The participants must be shown in the list of group call participants even if there is no information about them
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub participant_user_ids: Vec<i64>,
}

/// The verification state of an encrypted group call has changed; for group calls not bound to a chat only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallVerificationState {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// The call state generation to which the emoji corresponds. If generation is different for two users, then their emoji may be also different
    pub generation: i32,
    /// Group call state fingerprint represented as 4 emoji; may be empty if the state isn't verified yet
    pub emojis: Vec<String>,
}

/// A new message was received in a group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewGroupCallMessage {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// The message
    pub message: crate::types::GroupCallMessage,
}

/// A new paid reaction was received in a live story group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewGroupCallPaidReaction {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// Identifier of the sender of the reaction
    pub sender_id: crate::enums::MessageSender,
    /// The number of Telegram Stars that were paid to send the reaction
    pub star_count: i64,
}

/// A group call message failed to send
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallMessageSendFailed {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// Message identifier
    pub message_id: i32,
    /// The cause of the message sending failure
    pub error: crate::types::Error,
}

/// Some group call messages were deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallMessagesDeleted {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// Identifiers of the deleted messages
    pub message_ids: Vec<i32>,
}

/// New call signaling data arrived
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCallSignalingData {
    /// The call identifier
    pub call_id: i32,
    /// The data
    pub data: String,
}

/// The levels of live story group call messages have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallMessageLevels {
    /// New description of the levels in decreasing order of groupCallMessageLevel.min_star_count
    pub levels: Vec<crate::types::GroupCallMessageLevel>,
}

/// A new incoming callback query; for bots only
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCallbackQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// Identifier of the chat where the query was sent
    pub chat_id: i64,
    /// Identifier of the message from which the query originated
    pub message_id: i64,
    /// Identifier that uniquely corresponds to the chat to which the message was sent
    #[serde_as(as = "DisplayFromStr")]
    pub chat_instance: i64,
    /// Query payload
    pub payload: crate::enums::CallbackQueryPayload,
}

/// A new incoming callback query from a message sent via a bot; for bots only
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewInlineCallbackQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// Identifier of the inline message from which the query originated
    pub inline_message_id: String,
    /// An identifier uniquely corresponding to the chat a message was sent to
    #[serde_as(as = "DisplayFromStr")]
    pub chat_instance: i64,
    /// Query payload
    pub payload: crate::enums::CallbackQueryPayload,
}


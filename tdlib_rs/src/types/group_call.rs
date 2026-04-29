#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


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

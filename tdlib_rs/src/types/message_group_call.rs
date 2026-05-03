#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

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

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


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

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with information about an invitation to a video chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageInviteVideoChatParticipants {
    /// Identifier of the video chat. The video chat can be received through the method getGroupCall
    pub group_call_id: i32,
    /// Invited user identifiers
    pub user_ids: Vec<i64>,
}

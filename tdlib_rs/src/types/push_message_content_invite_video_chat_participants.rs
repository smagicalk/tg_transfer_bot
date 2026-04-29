#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An invitation of participants to a video chat or live stream
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentInviteVideoChatParticipants {
    /// True, if the current user was invited to the video chat or the live stream
    pub is_current_user: bool,
}

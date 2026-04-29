#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a video chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeVideoChat {
    /// Photo of the chat with the video chat; may be null if none
    pub photo: Option<crate::types::ChatPhoto>,
    /// True, if the video chat is expected to be a live stream in a channel or a broadcast group
    pub is_live_stream: bool,
    /// True, if the user can use the link to join the video chat without being muted by administrators
    pub joins_as_speaker: bool,
}

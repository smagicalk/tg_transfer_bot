#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinVideoChat with the given invite hash to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeVideoChat {
    /// Username of the chat with the video chat
    pub chat_username: String,
    /// If non-empty, invite hash to be used to join the video chat without being muted by administrators
    pub invite_hash: String,
    /// True, if the video chat is expected to be a live stream in a channel or a broadcast group
    pub is_live_stream: bool,
}

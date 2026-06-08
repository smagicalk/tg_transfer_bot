#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a link to a message or a forum topic in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageLinkInfo {
    /// True, if the link is a public link for a message or a forum topic in a chat
    pub is_public: bool,
    /// If found, identifier of the chat to which the link points, 0 otherwise
    pub chat_id: i64,
    /// Identifier of the specific topic in which the message must be opened, or a topic to open if the message is missing; may be null if none
    pub topic_id: Option<crate::enums::MessageTopic>,
    /// If found, the linked message; may be null
    pub message: Option<crate::types::Message>,
    /// Timestamp from which the video/audio/video note/voice note/story playing must start, in seconds; 0 if not specified. The media can be in the message content or in its link preview
    pub media_timestamp: i32,
    /// True, if the whole media album to which the message belongs is linked
    pub for_album: bool,
}

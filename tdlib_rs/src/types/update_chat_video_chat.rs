#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat video chat state has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatVideoChat {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of video_chat
    pub video_chat: crate::types::VideoChat,
}

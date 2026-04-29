#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A forwarded message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageForwarded {
    /// Identifier for the chat this forwarded message came from
    pub from_chat_id: i64,
    /// Identifier of the message to forward. A message can be forwarded only if messageProperties.can_be_forwarded
    pub message_id: i64,
    /// Pass true if a game message is being shared from a launched game; applies only to game messages
    pub in_game_share: bool,
    /// Pass true to replace video start timestamp in the forwarded message
    pub replace_video_start_timestamp: bool,
    /// The new video start timestamp; ignored if replace_video_start_timestamp == false
    pub new_video_start_timestamp: i32,
    /// Options to be used to copy content of the message without reference to the original sender; pass null to forward the message as usual
    pub copy_options: Option<crate::types::MessageCopyOptions>,
}

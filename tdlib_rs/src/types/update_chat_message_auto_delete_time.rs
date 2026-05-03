#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message auto-delete or self-destruct timer setting for a chat was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatMessageAutoDeleteTime {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of message_auto_delete_time
    pub message_auto_delete_time: i32,
}

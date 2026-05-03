#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The auto-delete or self-destruct timer for messages in the chat has been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatSetMessageAutoDeleteTime {
    /// New value auto-delete or self-destruct time, in seconds; 0 if disabled
    pub message_auto_delete_time: i32,
    /// If not 0, a user identifier, which default setting was automatically applied
    pub from_user_id: i64,
}

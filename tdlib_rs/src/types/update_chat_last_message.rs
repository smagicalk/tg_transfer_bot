#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The last message of a chat was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatLastMessage {
    /// Chat identifier
    pub chat_id: i64,
    /// The new last message in the chat; may be null if the last message became unknown. While the last message is unknown, new messages can be added to the chat without corresponding updateNewMessage update
    pub last_message: Option<crate::types::Message>,
    /// The new chat positions in the chat lists
    pub positions: Vec<crate::types::ChatPosition>,
}

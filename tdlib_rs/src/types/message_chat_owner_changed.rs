#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The owner of the chat has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatOwnerChanged {
    /// Identifier of the user who is the new owner of the chat
    pub new_owner_user_id: i64,
}

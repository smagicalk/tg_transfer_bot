#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The owner of the chat has left
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatOwnerLeft {
    /// Identifier of the user who will become the new owner of the chat if the previous owner isn't return; 0 if none
    pub new_owner_user_id: i64,
}

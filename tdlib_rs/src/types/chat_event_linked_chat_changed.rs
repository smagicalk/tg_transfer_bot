#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The linked chat of a supergroup was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventLinkedChatChanged {
    /// Previous supergroup linked chat identifier
    pub old_linked_chat_id: i64,
    /// New supergroup linked chat identifier
    pub new_linked_chat_id: i64,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatDraftMessage {
    /// Chat identifier
    pub chat_id: i64,
    /// The new draft message; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
    /// The new chat positions in the chat lists
    pub positions: Vec<crate::types::ChatPosition>,
}

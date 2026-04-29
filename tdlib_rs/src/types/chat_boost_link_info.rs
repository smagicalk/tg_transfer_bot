#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a link to boost a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostLinkInfo {
    /// True, if the link will work for non-members of the chat
    pub is_public: bool,
    /// Identifier of the chat to which the link points; 0 if the chat isn't found
    pub chat_id: i64,
}

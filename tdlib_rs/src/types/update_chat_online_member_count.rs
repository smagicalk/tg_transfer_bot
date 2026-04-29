#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The number of online group members has changed. This update with non-zero number of online group members is sent only for currently opened chats.
/// There is no guarantee that it is sent just after the number of online users has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatOnlineMemberCount {
    /// Identifier of the chat
    pub chat_id: i64,
    /// New number of online members in the chat, or 0 if unknown
    pub online_member_count: i32,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A list of chats added to a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatListFolder {
    /// Chat folder identifier
    pub chat_folder_id: i32,
}

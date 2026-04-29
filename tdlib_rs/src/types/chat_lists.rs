#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of chat lists
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatLists {
    /// List of chat lists
    pub chat_lists: Vec<crate::enums::ChatList>,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes private chats chosen for automatic interaction with a business
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessRecipients {
    /// Identifiers of selected private chats
    pub chat_ids: Vec<i64>,
    /// Identifiers of private chats that are always excluded; for businessConnectedBot only
    pub excluded_chat_ids: Vec<i64>,
    /// True, if all existing private chats are selected
    pub select_existing_chats: bool,
    /// True, if all new private chats are selected
    pub select_new_chats: bool,
    /// True, if all private chats with contacts are selected
    pub select_contacts: bool,
    /// True, if all private chats with non-contacts are selected
    pub select_non_contacts: bool,
    /// If true, then all private chats except the selected are chosen. Otherwise, only the selected chats are chosen
    pub exclude_selected: bool,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of quick reply shortcut messages has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateQuickReplyShortcutMessages {
    /// The identifier of the shortcut
    pub shortcut_id: i32,
    /// The new list of quick reply messages for the shortcut in order from the first to the last sent
    pub messages: Vec<crate::types::QuickReplyMessage>,
}

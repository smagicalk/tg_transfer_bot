#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat action bar was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatActionBar {
    /// Chat identifier
    pub chat_id: i64,
    /// The new value of the action bar; may be null
    pub action_bar: Option<crate::enums::ChatActionBar>,
}

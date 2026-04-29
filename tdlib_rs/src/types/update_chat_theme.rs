#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat theme was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatTheme {
    /// Chat identifier
    pub chat_id: i64,
    /// The new theme of the chat; may be null if theme was reset to default
    pub theme: Option<crate::enums::ChatTheme>,
}

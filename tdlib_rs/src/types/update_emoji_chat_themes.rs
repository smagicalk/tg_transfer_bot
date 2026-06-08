#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The list of available emoji chat themes has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateEmojiChatThemes {
    /// The new list of emoji chat themes
    pub chat_themes: Vec<crate::types::EmojiChatTheme>,
}

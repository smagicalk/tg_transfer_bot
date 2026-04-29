#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat theme based on an emoji
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatThemeEmoji {
    /// Name of the theme; full theme description is received through updateEmojiChatThemes
    pub name: String,
}

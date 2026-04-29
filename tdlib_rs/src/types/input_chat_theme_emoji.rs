#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A theme based on an emoji
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChatThemeEmoji {
    /// Name of the theme
    pub name: String,
}

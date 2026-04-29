#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes name of a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderName {
    /// The text of the chat folder name; 1-12 characters without line feeds. May contain only CustomEmoji entities
    pub text: crate::types::FormattedText,
    /// True, if custom emoji in the name must be animated
    pub animate_custom_emoji: bool,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A link to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockChatLink {
    /// Chat title
    pub title: String,
    /// Chat photo; may be null
    pub photo: Option<crate::types::ChatPhotoInfo>,
    /// Identifier of the accent color for chat title and background of chat photo
    pub accent_color_id: i32,
    /// Chat username by which all other information about the chat can be resolved
    pub username: String,
}

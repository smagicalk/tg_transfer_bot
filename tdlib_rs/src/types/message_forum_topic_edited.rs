#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A forum topic has been edited
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageForumTopicEdited {
    /// If non-empty, the new name of the topic
    pub name: String,
    /// True, if icon's custom_emoji_id is changed
    pub edit_icon_custom_emoji_id: bool,
    /// New unique identifier of the custom emoji shown on the topic icon; 0 if none. Must be ignored if edit_icon_custom_emoji_id is false
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
}

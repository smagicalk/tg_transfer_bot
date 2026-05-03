#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a forum topic icon
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ForumTopicIcon {
    /// Color of the topic icon in RGB format
    pub color: i32,
    /// Unique identifier of the custom emoji shown on the topic icon; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}

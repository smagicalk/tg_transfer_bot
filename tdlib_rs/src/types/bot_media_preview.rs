#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes media previews of a bot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BotMediaPreview {
    /// Point in time (Unix timestamp) when the preview was added or changed last time
    pub date: i32,
    /// Content of the preview; may only be of the types storyContentPhoto, storyContentVideo, or storyContentUnsupported
    pub content: crate::enums::StoryContent,
}

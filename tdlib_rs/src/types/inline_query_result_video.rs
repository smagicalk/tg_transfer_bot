#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a video
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultVideo {
    /// Unique identifier of the query result
    pub id: String,
    /// Video
    pub video: crate::types::Video,
    /// Title of the video
    pub title: String,
    /// Description of the video
    pub description: String,
}

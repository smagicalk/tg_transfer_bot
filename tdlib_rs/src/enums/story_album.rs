#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryAlbum {
    /// Describes album of stories
    #[serde(rename(serialize = "storyAlbum", deserialize = "storyAlbum"))]
    StoryAlbum(crate::types::StoryAlbum),
}

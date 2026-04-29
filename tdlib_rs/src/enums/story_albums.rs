#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryAlbums {
    /// Represents a list of story albums
    #[serde(rename(serialize = "storyAlbums", deserialize = "storyAlbums"))]
    StoryAlbums(crate::types::StoryAlbums),
}

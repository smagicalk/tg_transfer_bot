#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of story albums
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAlbums {
    /// List of story albums
    pub albums: Vec<crate::types::StoryAlbum>,
}

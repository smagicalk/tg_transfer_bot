#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes album of stories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryAlbum {
    /// Unique identifier of the album
    pub id: i32,
    /// Name of the album
    pub name: String,
    /// Icon of the album; may be null if none
    pub photo_icon: Option<crate::types::Photo>,
    /// Video icon of the album; may be null if none
    pub video_icon: Option<crate::types::Video>,
}

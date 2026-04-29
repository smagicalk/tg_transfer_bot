#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to an album of stories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeStoryAlbum {
    /// Icon of the album; may be null if none
    pub photo_icon: Option<crate::types::Photo>,
    /// Video icon of the album; may be null if none
    pub video_icon: Option<crate::types::Video>,
}

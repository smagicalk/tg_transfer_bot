#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a media album consisting of photos and videos
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeAlbum {
    /// The list of album media
    pub media: Vec<crate::enums::LinkPreviewAlbumMedia>,
    /// Album caption
    pub caption: String,
}

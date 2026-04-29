#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LinkPreviewAlbumMedia {
    /// The media is a photo
    #[serde(rename(serialize = "linkPreviewAlbumMediaPhoto", deserialize = "linkPreviewAlbumMediaPhoto"))]
    Photo(crate::types::LinkPreviewAlbumMediaPhoto),
    /// The media is a video
    #[serde(rename(serialize = "linkPreviewAlbumMediaVideo", deserialize = "linkPreviewAlbumMediaVideo"))]
    Video(crate::types::LinkPreviewAlbumMediaVideo),
}

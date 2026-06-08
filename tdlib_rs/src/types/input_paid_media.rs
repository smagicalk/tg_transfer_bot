#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a paid media to be sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPaidMedia {
    /// Type of the media
    pub r#type: crate::enums::InputPaidMediaType,
    /// Photo or video to be sent
    pub media: crate::enums::InputFile,
    /// Media thumbnail; pass null to skip thumbnail uploading
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// File identifiers of the stickers added to the media, if applicable
    pub added_sticker_file_ids: Vec<i32>,
    /// Media width
    pub width: i32,
    /// Media height
    pub height: i32,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A photo story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputStoryContentPhoto {
    /// Photo to send. The photo must be at most 10 MB in size. The photo size must be 1080x1920
    pub photo: crate::enums::InputFile,
    /// File identifiers of the stickers added to the photo, if applicable
    pub added_sticker_file_ids: Vec<i32>,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An animation message (GIF-style).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageAnimation {
    /// Animation file to be sent
    pub animation: crate::enums::InputFile,
    /// Animation thumbnail; pass null to skip thumbnail uploading
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// File identifiers of the stickers added to the animation, if applicable
    pub added_sticker_file_ids: Vec<i32>,
    /// Duration of the animation, in seconds
    pub duration: i32,
    /// Width of the animation; may be replaced by the server
    pub width: i32,
    /// Height of the animation; may be replaced by the server
    pub height: i32,
    /// Animation caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
    /// True, if the caption must be shown above the animation; otherwise, the caption must be shown below the animation; not supported in secret chats
    pub show_caption_above_media: bool,
    /// True, if the animation preview must be covered by a spoiler animation; not supported in secret chats
    pub has_spoiler: bool,
}

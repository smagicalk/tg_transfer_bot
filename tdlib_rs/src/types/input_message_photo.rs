#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A photo message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessagePhoto {
    /// Photo to send. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
    pub photo: crate::enums::InputFile,
    /// Photo thumbnail to be sent; pass null to skip thumbnail uploading. The thumbnail is sent to the other party only in secret chats
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// File identifiers of the stickers added to the photo, if applicable
    pub added_sticker_file_ids: Vec<i32>,
    /// Photo width
    pub width: i32,
    /// Photo height
    pub height: i32,
    /// Photo caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
    /// True, if the caption must be shown above the photo; otherwise, the caption must be shown below the photo; not supported in secret chats
    pub show_caption_above_media: bool,
    /// Photo self-destruct type; pass null if none; private chats only
    pub self_destruct_type: Option<crate::enums::MessageSelfDestructType>,
    /// True, if the photo preview must be covered by a spoiler animation; not supported in secret chats
    pub has_spoiler: bool,
}

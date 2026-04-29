#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A video message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageVideo {
    /// Video to be sent. The video is expected to be re-encoded to MPEG4 format with H.264 codec by the sender
    pub video: crate::enums::InputFile,
    /// Video thumbnail; pass null to skip thumbnail uploading
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// Cover of the video; pass null to skip cover uploading; not supported in secret chats and for self-destructing messages
    pub cover: Option<crate::enums::InputFile>,
    /// Timestamp from which the video playing must start, in seconds
    pub start_timestamp: i32,
    /// File identifiers of the stickers added to the video, if applicable
    pub added_sticker_file_ids: Vec<i32>,
    /// Duration of the video, in seconds
    pub duration: i32,
    /// Video width
    pub width: i32,
    /// Video height
    pub height: i32,
    /// True, if the video is expected to be streamed
    pub supports_streaming: bool,
    /// Video caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
    /// True, if the caption must be shown above the video; otherwise, the caption must be shown below the video; not supported in secret chats
    pub show_caption_above_media: bool,
    /// Video self-destruct type; pass null if none; private chats only
    pub self_destruct_type: Option<crate::enums::MessageSelfDestructType>,
    /// True, if the video preview must be covered by a spoiler animation; not supported in secret chats
    pub has_spoiler: bool,
}

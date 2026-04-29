#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A video note message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageVideoNote {
    /// Video note to be sent. The video is expected to be encoded to MPEG4 format with H.264 codec and have no data outside of the visible circle
    pub video_note: crate::enums::InputFile,
    /// Video thumbnail; may be null if empty; pass null to skip thumbnail uploading
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// Duration of the video, in seconds; 0-60
    pub duration: i32,
    /// Video width and height; must be positive and not greater than 640
    pub length: i32,
    /// Video note self-destruct type; may be null if none; pass null if none; private chats only
    pub self_destruct_type: Option<crate::enums::MessageSelfDestructType>,
}

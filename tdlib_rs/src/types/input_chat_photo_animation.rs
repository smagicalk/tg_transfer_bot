#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An animation in MPEG4 format; must be square, at most 10 seconds long, have width between 160 and 1280 and be at most 2MB in size
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputChatPhotoAnimation {
    /// Animation to be set as profile photo. Only inputFileLocal and inputFileGenerated are allowed
    pub animation: crate::enums::InputFile,
    /// Timestamp of the frame, which will be used as static chat photo
    pub main_frame_timestamp: f64,
}

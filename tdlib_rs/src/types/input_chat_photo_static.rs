#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A static photo in JPEG format
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputChatPhotoStatic {
    /// Photo to be set as profile photo. Only inputFileLocal and inputFileGenerated are allowed
    pub photo: crate::enums::InputFile,
}

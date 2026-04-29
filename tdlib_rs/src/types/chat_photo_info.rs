#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains basic information about the photo of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatPhotoInfo {
    /// A small (160x160) chat photo variant in JPEG format. The file can be downloaded only before the photo is changed
    pub small: crate::types::File,
    /// A big (640x640) chat photo variant in JPEG format. The file can be downloaded only before the photo is changed
    pub big: crate::types::File,
    /// Chat photo minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// True, if the photo has animated variant
    pub has_animation: bool,
    /// True, if the photo is visible only for the current user
    pub is_personal: bool,
}

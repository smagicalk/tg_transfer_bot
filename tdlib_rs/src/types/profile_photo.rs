#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a user profile photo
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProfilePhoto {
    /// Photo identifier; 0 for an empty photo. Can be used to find a photo in a list of user profile photos
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// A small (160x160) user profile photo. The file can be downloaded only before the photo is changed
    pub small: crate::types::File,
    /// A big (640x640) user profile photo. The file can be downloaded only before the photo is changed
    pub big: crate::types::File,
    /// User profile photo minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// True, if the photo has animated variant
    pub has_animation: bool,
    /// True, if the photo is visible only for the current user
    pub is_personal: bool,
}

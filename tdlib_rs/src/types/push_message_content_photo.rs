#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A photo message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentPhoto {
    /// Message content; may be null
    pub photo: Option<crate::types::Photo>,
    /// Photo caption
    pub caption: String,
    /// True, if the photo is secret
    pub is_secret: bool,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An animation message (GIF-style).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentAnimation {
    /// Message content; may be null
    pub animation: Option<crate::types::Animation>,
    /// Animation caption
    pub caption: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

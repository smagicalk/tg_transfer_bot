#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentSticker {
    /// Message content; may be null
    pub sticker: Option<crate::types::Sticker>,
    /// Emoji corresponding to the sticker; may be empty
    pub emoji: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

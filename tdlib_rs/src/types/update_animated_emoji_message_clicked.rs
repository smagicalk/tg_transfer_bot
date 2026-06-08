#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateAnimatedEmojiMessageClicked {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The animated sticker to be played
    pub sticker: crate::types::Sticker,
}

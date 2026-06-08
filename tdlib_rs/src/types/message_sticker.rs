#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A sticker message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageSticker {
    /// The sticker description
    pub sticker: crate::types::Sticker,
    /// True, if premium animation of the sticker must be played
    pub is_premium: bool,
}

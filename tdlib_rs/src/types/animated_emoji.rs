#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an animated or custom representation of an emoji
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AnimatedEmoji {
    /// Sticker for the emoji; may be null if yet unknown for a custom emoji. If the sticker is a custom emoji, then it can have arbitrary format
    pub sticker: Option<crate::types::Sticker>,
    /// Expected width of the sticker, which can be used if the sticker is null
    pub sticker_width: i32,
    /// Expected height of the sticker, which can be used if the sticker is null
    pub sticker_height: i32,
    /// Emoji modifier fitzpatrick type; 0-6; 0 if none
    pub fitzpatrick_type: i32,
    /// File containing the sound to be played when the sticker is clicked; may be null. The sound is encoded with the Opus codec, and stored inside an OGG container
    pub sound: Option<crate::types::File>,
}

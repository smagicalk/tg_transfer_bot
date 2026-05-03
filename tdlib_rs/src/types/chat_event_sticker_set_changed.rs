#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The supergroup sticker set was changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventStickerSetChanged {
    /// Previous identifier of the chat sticker set; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub old_sticker_set_id: i64,
    /// New identifier of the chat sticker set; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub new_sticker_set_id: i64,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of recently used stickers was updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateRecentStickers {
    /// True, if the list of stickers attached to photo or video files was updated; otherwise, the list of sent stickers is updated
    pub is_attached: bool,
    /// The new list of file identifiers of recently used stickers
    pub sticker_ids: Vec<i32>,
}

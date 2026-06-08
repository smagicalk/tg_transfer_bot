#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The list of favorite stickers was updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateFavoriteStickers {
    /// The new list of file identifiers of favorite stickers
    pub sticker_ids: Vec<i32>,
}

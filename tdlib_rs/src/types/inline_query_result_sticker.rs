#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultSticker {
    /// Unique identifier of the query result
    pub id: String,
    /// Sticker
    pub sticker: crate::types::Sticker,
}

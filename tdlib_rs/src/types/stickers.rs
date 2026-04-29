#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of stickers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Stickers {
    /// List of stickers
    pub stickers: Vec<crate::types::Sticker>,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputSticker {
    /// A sticker to be added to a sticker set
    #[serde(rename(serialize = "inputSticker", deserialize = "inputSticker"))]
    InputSticker(crate::types::InputSticker),
}

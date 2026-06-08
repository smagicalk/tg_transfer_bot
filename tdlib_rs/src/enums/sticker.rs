#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Sticker {
    /// Describes a sticker
    #[serde(rename(serialize = "sticker", deserialize = "sticker"))]
    Sticker(crate::types::Sticker),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Stickers {
    /// Represents a list of stickers
    #[serde(rename(serialize = "stickers", deserialize = "stickers"))]
    Stickers(crate::types::Stickers),
}

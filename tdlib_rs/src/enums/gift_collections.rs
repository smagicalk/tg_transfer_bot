#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftCollections {
    /// Contains a list of gift collections
    #[serde(rename(serialize = "giftCollections", deserialize = "giftCollections"))]
    GiftCollections(crate::types::GiftCollections),
}

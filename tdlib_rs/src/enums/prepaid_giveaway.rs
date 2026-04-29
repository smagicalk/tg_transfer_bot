#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PrepaidGiveaway {
    /// Describes a prepaid giveaway
    #[serde(rename(serialize = "prepaidGiveaway", deserialize = "prepaidGiveaway"))]
    PrepaidGiveaway(crate::types::PrepaidGiveaway),
}

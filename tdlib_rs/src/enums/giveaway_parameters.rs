#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiveawayParameters {
    /// Describes parameters of a giveaway
    #[serde(rename(serialize = "giveawayParameters", deserialize = "giveawayParameters"))]
    GiveawayParameters(crate::types::GiveawayParameters),
}

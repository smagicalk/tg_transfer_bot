#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiveawayInfo {
    /// Describes an ongoing giveaway
    #[serde(rename(serialize = "giveawayInfoOngoing", deserialize = "giveawayInfoOngoing"))]
    Ongoing(crate::types::GiveawayInfoOngoing),
    /// Describes a completed giveaway
    #[serde(rename(
        serialize = "giveawayInfoCompleted",
        deserialize = "giveawayInfoCompleted"
    ))]
    Completed(crate::types::GiveawayInfoCompleted),
}

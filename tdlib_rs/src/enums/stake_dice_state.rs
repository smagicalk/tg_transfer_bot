#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StakeDiceState {
    /// Describes state of the stake dice
    #[serde(rename(serialize = "stakeDiceState", deserialize = "stakeDiceState"))]
    StakeDiceState(crate::types::StakeDiceState),
}

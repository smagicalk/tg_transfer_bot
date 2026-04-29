#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessAwayMessageSettings {
    /// Describes settings for messages that are automatically sent by a Telegram Business account when it is away
    #[serde(rename(serialize = "businessAwayMessageSettings", deserialize = "businessAwayMessageSettings"))]
    BusinessAwayMessageSettings(crate::types::BusinessAwayMessageSettings),
}

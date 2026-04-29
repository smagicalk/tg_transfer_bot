#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessConnectedBot {
    /// Describes a bot connected to a business account
    #[serde(rename(serialize = "businessConnectedBot", deserialize = "businessConnectedBot"))]
    BusinessConnectedBot(crate::types::BusinessConnectedBot),
}

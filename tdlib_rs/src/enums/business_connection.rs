#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessConnection {
    /// Describes a connection of the bot with a business account
    #[serde(rename(serialize = "businessConnection", deserialize = "businessConnection"))]
    BusinessConnection(crate::types::BusinessConnection),
}

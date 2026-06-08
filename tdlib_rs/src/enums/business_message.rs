#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessMessage {
    /// Describes a message from a business account as received by a bot
    #[serde(rename(serialize = "businessMessage", deserialize = "businessMessage"))]
    BusinessMessage(crate::types::BusinessMessage),
}

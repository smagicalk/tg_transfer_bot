#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessMessages {
    /// Contains a list of messages from a business account as received by a bot
    #[serde(rename(serialize = "businessMessages", deserialize = "businessMessages"))]
    BusinessMessages(crate::types::BusinessMessages),
}

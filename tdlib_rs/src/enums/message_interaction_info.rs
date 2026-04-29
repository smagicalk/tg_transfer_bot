#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageInteractionInfo {
    /// Contains information about interactions with a message
    #[serde(rename(serialize = "messageInteractionInfo", deserialize = "messageInteractionInfo"))]
    MessageInteractionInfo(crate::types::MessageInteractionInfo),
}

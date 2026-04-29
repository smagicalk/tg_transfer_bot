#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageForwardInfo {
    /// Contains information about a forwarded message
    #[serde(rename(serialize = "messageForwardInfo", deserialize = "messageForwardInfo"))]
    MessageForwardInfo(crate::types::MessageForwardInfo),
}

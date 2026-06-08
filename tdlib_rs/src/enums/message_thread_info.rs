#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageThreadInfo {
    /// Contains information about a message thread
    #[serde(rename(serialize = "messageThreadInfo", deserialize = "messageThreadInfo"))]
    MessageThreadInfo(crate::types::MessageThreadInfo),
}

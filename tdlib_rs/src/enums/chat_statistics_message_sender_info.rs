#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsMessageSenderInfo {
    /// Contains statistics about messages sent by a user
    #[serde(rename(serialize = "chatStatisticsMessageSenderInfo", deserialize = "chatStatisticsMessageSenderInfo"))]
    ChatStatisticsMessageSenderInfo(crate::types::ChatStatisticsMessageSenderInfo),
}

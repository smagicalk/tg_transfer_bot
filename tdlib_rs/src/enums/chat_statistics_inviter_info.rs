#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsInviterInfo {
    /// Contains statistics about number of new members invited by a user
    #[serde(rename(serialize = "chatStatisticsInviterInfo", deserialize = "chatStatisticsInviterInfo"))]
    ChatStatisticsInviterInfo(crate::types::ChatStatisticsInviterInfo),
}

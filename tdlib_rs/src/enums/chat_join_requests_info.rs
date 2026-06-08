#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatJoinRequestsInfo {
    /// Contains information about pending join requests for a chat
    #[serde(rename(
        serialize = "chatJoinRequestsInfo",
        deserialize = "chatJoinRequestsInfo"
    ))]
    ChatJoinRequestsInfo(crate::types::ChatJoinRequestsInfo),
}

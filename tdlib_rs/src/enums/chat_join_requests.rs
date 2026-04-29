#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatJoinRequests {
    /// Contains a list of requests to join a chat
    #[serde(rename(serialize = "chatJoinRequests", deserialize = "chatJoinRequests"))]
    ChatJoinRequests(crate::types::ChatJoinRequests),
}

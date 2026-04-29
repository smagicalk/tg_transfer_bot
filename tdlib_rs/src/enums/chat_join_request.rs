#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatJoinRequest {
    /// Describes a user who sent a join request and waits for administrator approval
    #[serde(rename(serialize = "chatJoinRequest", deserialize = "chatJoinRequest"))]
    ChatJoinRequest(crate::types::ChatJoinRequest),
}

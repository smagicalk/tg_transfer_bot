#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMembers {
    /// Contains a list of chat members
    #[serde(rename(serialize = "chatMembers", deserialize = "chatMembers"))]
    ChatMembers(crate::types::ChatMembers),
}

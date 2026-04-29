#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SharedChat {
    /// Contains information about a chat shared with a bot
    #[serde(rename(serialize = "sharedChat", deserialize = "sharedChat"))]
    SharedChat(crate::types::SharedChat),
}

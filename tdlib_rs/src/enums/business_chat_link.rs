#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessChatLink {
    /// Contains information about a business chat link
    #[serde(rename(serialize = "businessChatLink", deserialize = "businessChatLink"))]
    BusinessChatLink(crate::types::BusinessChatLink),
}

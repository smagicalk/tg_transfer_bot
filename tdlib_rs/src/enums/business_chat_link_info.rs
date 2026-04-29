#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessChatLinkInfo {
    /// Contains information about a business chat link
    #[serde(rename(serialize = "businessChatLinkInfo", deserialize = "businessChatLinkInfo"))]
    BusinessChatLinkInfo(crate::types::BusinessChatLinkInfo),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessChatLinks {
    /// Contains a list of business chat links created by the user
    #[serde(rename(serialize = "businessChatLinks", deserialize = "businessChatLinks"))]
    BusinessChatLinks(crate::types::BusinessChatLinks),
}

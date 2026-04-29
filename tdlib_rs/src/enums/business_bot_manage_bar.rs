#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessBotManageBar {
    /// Contains information about a business bot that manages the chat
    #[serde(rename(serialize = "businessBotManageBar", deserialize = "businessBotManageBar"))]
    BusinessBotManageBar(crate::types::BusinessBotManageBar),
}

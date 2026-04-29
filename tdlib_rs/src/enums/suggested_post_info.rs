#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedPostInfo {
    /// Contains information about a suggested post. If the post can be approved or declined, then changes to the post can be also suggested. Use sendMessage with reply to the message
    /// and suggested post information to suggest message changes. Use addOffer to suggest price or time changes
    #[serde(rename(serialize = "suggestedPostInfo", deserialize = "suggestedPostInfo"))]
    SuggestedPostInfo(crate::types::SuggestedPostInfo),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum QuickReplyMessages {
    /// Contains a list of quick reply messages
    #[serde(rename(serialize = "quickReplyMessages", deserialize = "quickReplyMessages"))]
    QuickReplyMessages(crate::types::QuickReplyMessages),
}

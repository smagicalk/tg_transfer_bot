#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum QuickReplyShortcut {
    /// Describes a shortcut that can be used for a quick reply
    #[serde(rename(serialize = "quickReplyShortcut", deserialize = "quickReplyShortcut"))]
    QuickReplyShortcut(crate::types::QuickReplyShortcut),
}

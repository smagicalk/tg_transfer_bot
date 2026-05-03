#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UnreadReaction {
    /// Contains information about an unread reaction to a message
    #[serde(rename(serialize = "unreadReaction", deserialize = "unreadReaction"))]
    UnreadReaction(crate::types::UnreadReaction),
}

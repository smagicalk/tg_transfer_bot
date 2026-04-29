#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about interactions with a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageInteractionInfo {
    /// Number of times the message was viewed
    pub view_count: i32,
    /// Number of times the message was forwarded
    pub forward_count: i32,
    /// Information about direct or indirect replies to the message; may be null. Currently, available only in channels with a discussion supergroup and discussion supergroups for messages, which are not replies itself
    pub reply_info: Option<crate::types::MessageReplyInfo>,
    /// The list of reactions or tags added to the message; may be null
    pub reactions: Option<crate::types::MessageReactions>,
}

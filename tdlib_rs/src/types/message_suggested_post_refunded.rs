#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A suggested post was refunded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostRefunded {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// Reason of the refund
    pub reason: crate::enums::SuggestedPostRefundReason,
}

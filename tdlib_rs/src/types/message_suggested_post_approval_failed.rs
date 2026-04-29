#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Approval of suggested post has failed, because the user which proposed the post had no enough funds
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostApprovalFailed {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// Price of the suggested post
    pub price: crate::enums::SuggestedPostPrice,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A suggested post was approved
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostApproved {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// Price of the suggested post; may be null if the post is non-paid
    pub price: Option<crate::enums::SuggestedPostPrice>,
    /// Point in time (Unix timestamp) when the post is expected to be published
    pub send_date: i32,
}

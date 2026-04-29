#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A suggested post was declined
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostDeclined {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// Comment added by administrator of the channel when the post was declined
    pub comment: String,
}

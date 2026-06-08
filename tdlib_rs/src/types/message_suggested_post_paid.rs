#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A suggested post was published for getOption("suggested_post_lifetime_min") seconds and payment for the post was received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostPaid {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// The amount of received Telegram Stars
    pub star_amount: crate::types::StarAmount,
    /// The amount of received Toncoins; in the smallest units of the cryptocurrency
    pub ton_amount: i64,
}

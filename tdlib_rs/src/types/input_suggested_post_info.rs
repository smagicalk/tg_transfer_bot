#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a post to suggest
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputSuggestedPostInfo {
    /// Price of the suggested post; pass null to suggest a post without payment. If the current user isn't an administrator of the channel direct messages chat
    /// and has no enough funds to pay for the post, then the error "BALANCE_TOO_LOW" will be returned immediately
    pub price: Option<crate::enums::SuggestedPostPrice>,
    /// Point in time (Unix timestamp) when the post is expected to be published; pass 0 if the date isn't restricted. If specified,
    /// then the date must be getOption("suggested_post_send_delay_min")-getOption("suggested_post_send_delay_max") seconds in the future
    pub send_date: i32,
}

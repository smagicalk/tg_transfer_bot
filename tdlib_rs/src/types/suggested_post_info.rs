#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a suggested post. If the post can be approved or declined, then changes to the post can be also suggested. Use sendMessage with reply to the message
/// and suggested post information to suggest message changes. Use addOffer to suggest price or time changes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostInfo {
    /// Price of the suggested post; may be null if the post is non-paid
    pub price: Option<crate::enums::SuggestedPostPrice>,
    /// Point in time (Unix timestamp) when the post is expected to be published; 0 if the specific date isn't set yet
    pub send_date: i32,
    /// State of the post
    pub state: crate::enums::SuggestedPostState,
    /// True, if the suggested post can be approved by the current user using approveSuggestedPost; updates aren't sent when value of this field changes
    pub can_be_approved: bool,
    /// True, if the suggested post can be declined by the current user using declineSuggestedPost; updates aren't sent when value of this field changes
    pub can_be_declined: bool,
}

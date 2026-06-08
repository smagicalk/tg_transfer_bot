#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a gift auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeGiftAuction {
    /// The gift
    pub gift: crate::types::Gift,
    /// Point in time (Unix timestamp) when the auction will be ended
    pub auction_end_date: i32,
}

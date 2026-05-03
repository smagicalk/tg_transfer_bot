#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An offer to purchase an upgraded gift was sent or received
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageUpgradedGiftPurchaseOffer {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// State of the offer
    pub state: crate::enums::GiftPurchaseOfferState,
    /// The proposed price
    pub price: crate::enums::GiftResalePrice,
    /// Point in time (Unix timestamp) when the offer will expire or has expired
    pub expiration_date: i32,
}

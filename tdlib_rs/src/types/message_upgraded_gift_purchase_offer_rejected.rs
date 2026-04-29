#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An offer to purchase a gift was rejected or expired
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageUpgradedGiftPurchaseOfferRejected {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// The proposed price
    pub price: crate::enums::GiftResalePrice,
    /// Identifier of the message with purchase offer which was rejected or expired; may be 0 or an identifier of a deleted message
    pub offer_message_id: i64,
    /// True, if the offer has expired; otherwise, the offer was explicitly rejected
    pub was_expired: bool,
}

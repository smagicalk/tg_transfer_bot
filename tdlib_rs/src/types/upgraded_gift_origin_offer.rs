#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The gift was bought through an offer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginOffer {
    /// Price paid for the gift
    pub price: crate::enums::GiftResalePrice,
}

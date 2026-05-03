#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The gift was bought from another user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginResale {
    /// Price paid for the gift
    pub price: crate::enums::GiftResalePrice,
}

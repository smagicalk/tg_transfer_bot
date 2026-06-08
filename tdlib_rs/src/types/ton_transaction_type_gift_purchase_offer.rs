#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is an offer of gift purchase
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeGiftPurchaseOffer {
    /// The gift
    pub gift: crate::types::UpgradedGift,
}

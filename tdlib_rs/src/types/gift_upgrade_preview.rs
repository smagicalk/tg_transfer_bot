#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains examples of possible upgraded gifts for the given regular gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftUpgradePreview {
    /// Examples of possible models that can be chosen for the gift after upgrade
    pub models: Vec<crate::types::UpgradedGiftModel>,
    /// Examples of possible symbols that can be chosen for the gift after upgrade
    pub symbols: Vec<crate::types::UpgradedGiftSymbol>,
    /// Examples of possible backdrops that can be chosen for the gift after upgrade
    pub backdrops: Vec<crate::types::UpgradedGiftBackdrop>,
    /// Examples of price for gift upgrade from the maximum price to the minimum price
    pub prices: Vec<crate::types::GiftUpgradePrice>,
    /// Next changes for the price for gift upgrade with more granularity than in prices
    pub next_prices: Vec<crate::types::GiftUpgradePrice>,
}

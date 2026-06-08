#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains all possible variants of upgraded gifts for the given regular gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftUpgradeVariants {
    /// Models that can be chosen for the gift after upgrade
    pub models: Vec<crate::types::UpgradedGiftModel>,
    /// Symbols that can be chosen for the gift after upgrade
    pub symbols: Vec<crate::types::UpgradedGiftSymbol>,
    /// Backdrops that can be chosen for the gift after upgrade
    pub backdrops: Vec<crate::types::UpgradedGiftBackdrop>,
}

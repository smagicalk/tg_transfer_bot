#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftOrigin {
    /// The gift was obtained by upgrading of a previously received gift
    #[serde(rename(
        serialize = "upgradedGiftOriginUpgrade",
        deserialize = "upgradedGiftOriginUpgrade"
    ))]
    Upgrade(crate::types::UpgradedGiftOriginUpgrade),
    /// The gift was transferred from another owner
    #[serde(rename(
        serialize = "upgradedGiftOriginTransfer",
        deserialize = "upgradedGiftOriginTransfer"
    ))]
    Transfer,
    /// The gift was bought from another user
    #[serde(rename(
        serialize = "upgradedGiftOriginResale",
        deserialize = "upgradedGiftOriginResale"
    ))]
    Resale(crate::types::UpgradedGiftOriginResale),
    /// The gift was assigned from blockchain and isn't owned by the current user. The gift can't be transferred, resold or withdrawn to blockchain
    #[serde(rename(
        serialize = "upgradedGiftOriginBlockchain",
        deserialize = "upgradedGiftOriginBlockchain"
    ))]
    Blockchain,
    /// The sender or receiver of the message has paid for upgraid of the gift, which has been completed
    #[serde(rename(
        serialize = "upgradedGiftOriginPrepaidUpgrade",
        deserialize = "upgradedGiftOriginPrepaidUpgrade"
    ))]
    PrepaidUpgrade,
    /// The gift was bought through an offer
    #[serde(rename(
        serialize = "upgradedGiftOriginOffer",
        deserialize = "upgradedGiftOriginOffer"
    ))]
    Offer(crate::types::UpgradedGiftOriginOffer),
    /// The gift was crafted from other gifts
    #[serde(rename(
        serialize = "upgradedGiftOriginCraft",
        deserialize = "upgradedGiftOriginCraft"
    ))]
    Craft,
}

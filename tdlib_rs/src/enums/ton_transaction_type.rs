#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TonTransactionType {
    /// The transaction is a deposit of Toncoins from Fragment
    #[serde(rename(serialize = "tonTransactionTypeFragmentDeposit", deserialize = "tonTransactionTypeFragmentDeposit"))]
    FragmentDeposit(crate::types::TonTransactionTypeFragmentDeposit),
    /// The transaction is a withdrawal of earned Toncoins to Fragment
    #[serde(rename(serialize = "tonTransactionTypeFragmentWithdrawal", deserialize = "tonTransactionTypeFragmentWithdrawal"))]
    FragmentWithdrawal(crate::types::TonTransactionTypeFragmentWithdrawal),
    /// The transaction is a payment for a suggested post
    #[serde(rename(serialize = "tonTransactionTypeSuggestedPostPayment", deserialize = "tonTransactionTypeSuggestedPostPayment"))]
    SuggestedPostPayment(crate::types::TonTransactionTypeSuggestedPostPayment),
    /// The transaction is an offer of gift purchase
    #[serde(rename(serialize = "tonTransactionTypeGiftPurchaseOffer", deserialize = "tonTransactionTypeGiftPurchaseOffer"))]
    GiftPurchaseOffer(crate::types::TonTransactionTypeGiftPurchaseOffer),
    /// The transaction is a purchase of an upgraded gift for some user or channel
    #[serde(rename(serialize = "tonTransactionTypeUpgradedGiftPurchase", deserialize = "tonTransactionTypeUpgradedGiftPurchase"))]
    UpgradedGiftPurchase(crate::types::TonTransactionTypeUpgradedGiftPurchase),
    /// The transaction is a sale of an upgraded gift
    #[serde(rename(serialize = "tonTransactionTypeUpgradedGiftSale", deserialize = "tonTransactionTypeUpgradedGiftSale"))]
    UpgradedGiftSale(crate::types::TonTransactionTypeUpgradedGiftSale),
    /// The transaction is a payment for stake dice throw
    #[serde(rename(serialize = "tonTransactionTypeStakeDiceStake", deserialize = "tonTransactionTypeStakeDiceStake"))]
    StakeDiceStake,
    /// The transaction is a payment for successful stake dice throw
    #[serde(rename(serialize = "tonTransactionTypeStakeDicePayout", deserialize = "tonTransactionTypeStakeDicePayout"))]
    StakeDicePayout,
    /// The transaction is a transaction of an unsupported type
    #[serde(rename(serialize = "tonTransactionTypeUnsupported", deserialize = "tonTransactionTypeUnsupported"))]
    Unsupported,
}

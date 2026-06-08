#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatRevenueTransactionType {
    /// Describes an unsupported transaction
    #[serde(rename(
        serialize = "chatRevenueTransactionTypeUnsupported",
        deserialize = "chatRevenueTransactionTypeUnsupported"
    ))]
    Unsupported,
    /// Describes earnings from sponsored messages in a chat in some time frame
    #[serde(rename(
        serialize = "chatRevenueTransactionTypeSponsoredMessageEarnings",
        deserialize = "chatRevenueTransactionTypeSponsoredMessageEarnings"
    ))]
    SponsoredMessageEarnings(crate::types::ChatRevenueTransactionTypeSponsoredMessageEarnings),
    /// Describes earnings from a published suggested post
    #[serde(rename(
        serialize = "chatRevenueTransactionTypeSuggestedPostEarnings",
        deserialize = "chatRevenueTransactionTypeSuggestedPostEarnings"
    ))]
    SuggestedPostEarnings(crate::types::ChatRevenueTransactionTypeSuggestedPostEarnings),
    /// Describes a withdrawal of earnings through Fragment
    #[serde(rename(
        serialize = "chatRevenueTransactionTypeFragmentWithdrawal",
        deserialize = "chatRevenueTransactionTypeFragmentWithdrawal"
    ))]
    FragmentWithdrawal(crate::types::ChatRevenueTransactionTypeFragmentWithdrawal),
    /// Describes a refund for failed withdrawal of earnings through Fragment
    #[serde(rename(
        serialize = "chatRevenueTransactionTypeFragmentRefund",
        deserialize = "chatRevenueTransactionTypeFragmentRefund"
    ))]
    FragmentRefund(crate::types::ChatRevenueTransactionTypeFragmentRefund),
}

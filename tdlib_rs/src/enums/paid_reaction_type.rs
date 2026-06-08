#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaidReactionType {
    /// A paid reaction on behalf of the current user
    #[serde(rename(
        serialize = "paidReactionTypeRegular",
        deserialize = "paidReactionTypeRegular"
    ))]
    Regular,
    /// An anonymous paid reaction
    #[serde(rename(
        serialize = "paidReactionTypeAnonymous",
        deserialize = "paidReactionTypeAnonymous"
    ))]
    Anonymous,
    /// A paid reaction on behalf of an owned chat
    #[serde(rename(
        serialize = "paidReactionTypeChat",
        deserialize = "paidReactionTypeChat"
    ))]
    Chat(crate::types::PaidReactionTypeChat),
}

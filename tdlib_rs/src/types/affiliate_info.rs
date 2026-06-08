#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about an affiliate that received commission from a Telegram Star transaction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateInfo {
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner
    pub commission_per_mille: i32,
    /// Identifier of the chat which received the commission
    pub affiliate_chat_id: i64,
    /// The Telegram Star amount that was received by the affiliate; can be negative for refunds
    pub star_amount: crate::types::StarAmount,
}

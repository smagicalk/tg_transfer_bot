#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Premium gift code was created for the user
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessagePremiumGiftCode {
    /// Identifier of a chat or a user who created the gift code; may be null if unknown
    pub creator_id: Option<crate::enums::MessageSender>,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// True, if the gift code was created for a giveaway
    pub is_from_giveaway: bool,
    /// True, if the winner for the corresponding Telegram Premium subscription wasn't chosen
    pub is_unclaimed: bool,
    /// Currency for the paid amount; empty if unknown
    pub currency: String,
    /// The paid amount, in the smallest units of the currency; 0 if unknown
    pub amount: i64,
    /// Cryptocurrency used to pay for the gift; may be empty if none or unknown
    pub cryptocurrency: String,
    /// The paid amount, in the smallest units of the cryptocurrency; 0 if unknown
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Number of months the Telegram Premium subscription will be active after code activation; 0 if the number of months isn't integer
    pub month_count: i32,
    /// Number of days the Telegram Premium subscription will be active after code activation
    pub day_count: i32,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
    /// The gift code
    pub code: String,
}

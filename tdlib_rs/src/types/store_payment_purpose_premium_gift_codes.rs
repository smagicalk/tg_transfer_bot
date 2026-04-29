#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user boosting a chat by creating Telegram Premium gift codes for other users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposePremiumGiftCodes {
    /// Identifier of the supergroup or channel chat, which will be automatically boosted by the users for duration of the Premium subscription and which is administered by the user
    pub boosted_chat_id: i64,
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Identifiers of the users which can activate the gift codes
    pub user_ids: Vec<i64>,
    /// Text to show along with the gift codes; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed
    pub text: crate::types::FormattedText,
}

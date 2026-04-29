#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user gifting Telegram Premium to another user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposePremiumGift {
    /// ISO 4217 currency code of the payment currency
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Identifiers of the user which will receive Telegram Premium
    pub user_id: i64,
    /// Text to show along with the gift codes; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed
    pub text: crate::types::FormattedText,
}

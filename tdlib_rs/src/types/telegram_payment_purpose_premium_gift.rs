#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user gifting Telegram Premium to another user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TelegramPaymentPurposePremiumGift {
    /// ISO 4217 currency code of the payment currency, or "XTR" for payments in Telegram Stars
    pub currency: String,
    /// Paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Identifier of the user which will receive Telegram Premium
    pub user_id: i64,
    /// Number of months the Telegram Premium subscription will be active for the user
    pub month_count: i32,
    /// Text to show to the user receiving Telegram Premium; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed
    pub text: crate::types::FormattedText,
}

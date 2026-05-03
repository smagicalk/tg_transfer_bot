#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An invoice for a payment toward Telegram; must not be used in the in-store apps
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInvoiceTelegram {
    /// Transaction purpose
    pub purpose: crate::enums::TelegramPaymentPurpose,
}

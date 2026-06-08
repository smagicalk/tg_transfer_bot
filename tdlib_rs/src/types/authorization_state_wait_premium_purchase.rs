#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user must buy Telegram Premium as an in-store purchase to log in. Call checkAuthenticationPremiumPurchase and then setAuthenticationPremiumPurchaseTransaction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitPremiumPurchase {
    /// Identifier of the store product that must be bought
    pub store_product_id: String,
    /// Email address to use for support if the user has issues with Telegram Premium purchase
    pub support_email_address: String,
    /// Subject for the email sent to the support email address
    pub support_email_subject: String,
}

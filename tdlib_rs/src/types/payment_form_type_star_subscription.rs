#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The payment form is for a payment in Telegram Stars for subscription
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentFormTypeStarSubscription {
    /// Information about subscription plan
    pub pricing: crate::types::StarSubscriptionPricing,
}

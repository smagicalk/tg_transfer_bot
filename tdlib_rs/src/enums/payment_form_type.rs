#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentFormType {
    /// The payment form is for a regular payment
    #[serde(rename(
        serialize = "paymentFormTypeRegular",
        deserialize = "paymentFormTypeRegular"
    ))]
    Regular(crate::types::PaymentFormTypeRegular),
    /// The payment form is for a payment in Telegram Stars
    #[serde(rename(
        serialize = "paymentFormTypeStars",
        deserialize = "paymentFormTypeStars"
    ))]
    Stars(crate::types::PaymentFormTypeStars),
    /// The payment form is for a payment in Telegram Stars for subscription
    #[serde(rename(
        serialize = "paymentFormTypeStarSubscription",
        deserialize = "paymentFormTypeStarSubscription"
    ))]
    StarSubscription(crate::types::PaymentFormTypeStarSubscription),
}

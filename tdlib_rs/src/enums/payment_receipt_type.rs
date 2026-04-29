#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentReceiptType {
    /// The payment was done using a third-party payment provider
    #[serde(rename(serialize = "paymentReceiptTypeRegular", deserialize = "paymentReceiptTypeRegular"))]
    Regular(crate::types::PaymentReceiptTypeRegular),
    /// The payment was done using Telegram Stars
    #[serde(rename(serialize = "paymentReceiptTypeStars", deserialize = "paymentReceiptTypeStars"))]
    Stars(crate::types::PaymentReceiptTypeStars),
}

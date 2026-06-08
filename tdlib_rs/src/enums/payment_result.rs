#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentResult {
    /// Contains the result of a payment request
    #[serde(rename(serialize = "paymentResult", deserialize = "paymentResult"))]
    PaymentResult(crate::types::PaymentResult),
}

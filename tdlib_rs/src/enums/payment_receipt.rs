#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentReceipt {
    /// Contains information about a successful payment
    #[serde(rename(serialize = "paymentReceipt", deserialize = "paymentReceipt"))]
    PaymentReceipt(crate::types::PaymentReceipt),
}

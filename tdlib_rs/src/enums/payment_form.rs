#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentForm {
    /// Contains information about an invoice payment form
    #[serde(rename(serialize = "paymentForm", deserialize = "paymentForm"))]
    PaymentForm(crate::types::PaymentForm),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentOption {
    /// Describes an additional payment option
    #[serde(rename(serialize = "paymentOption", deserialize = "paymentOption"))]
    PaymentOption(crate::types::PaymentOption),
}

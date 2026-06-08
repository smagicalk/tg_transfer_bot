#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ShippingOption {
    /// One shipping option
    #[serde(rename(serialize = "shippingOption", deserialize = "shippingOption"))]
    ShippingOption(crate::types::ShippingOption),
}

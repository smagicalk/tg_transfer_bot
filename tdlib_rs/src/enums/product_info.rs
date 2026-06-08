#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ProductInfo {
    /// Contains information about a product that can be paid with invoice
    #[serde(rename(serialize = "productInfo", deserialize = "productInfo"))]
    ProductInfo(crate::types::ProductInfo),
}

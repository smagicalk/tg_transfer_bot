#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum OrderInfo {
    /// Order information
    #[serde(rename(serialize = "orderInfo", deserialize = "orderInfo"))]
    OrderInfo(crate::types::OrderInfo),
}

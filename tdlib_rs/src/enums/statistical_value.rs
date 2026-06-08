#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StatisticalValue {
    /// A value with information about its recent changes
    #[serde(rename(serialize = "statisticalValue", deserialize = "statisticalValue"))]
    StatisticalValue(crate::types::StatisticalValue),
}

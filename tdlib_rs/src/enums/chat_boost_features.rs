#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostFeatures {
    /// Contains a list of features available on the first chat boost levels
    #[serde(rename(serialize = "chatBoostFeatures", deserialize = "chatBoostFeatures"))]
    ChatBoostFeatures(crate::types::ChatBoostFeatures),
}

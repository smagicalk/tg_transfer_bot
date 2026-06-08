#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostLevelFeatures {
    /// Contains a list of features available on a specific chat boost level
    #[serde(rename(
        serialize = "chatBoostLevelFeatures",
        deserialize = "chatBoostLevelFeatures"
    ))]
    ChatBoostLevelFeatures(crate::types::ChatBoostLevelFeatures),
}

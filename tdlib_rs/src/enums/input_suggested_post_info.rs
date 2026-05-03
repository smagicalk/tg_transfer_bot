#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputSuggestedPostInfo {
    /// Contains information about a post to suggest
    #[serde(rename(
        serialize = "inputSuggestedPostInfo",
        deserialize = "inputSuggestedPostInfo"
    ))]
    InputSuggestedPostInfo(crate::types::InputSuggestedPostInfo),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RecommendedChatFolder {
    /// Describes a recommended chat folder
    #[serde(rename(
        serialize = "recommendedChatFolder",
        deserialize = "recommendedChatFolder"
    ))]
    RecommendedChatFolder(crate::types::RecommendedChatFolder),
}

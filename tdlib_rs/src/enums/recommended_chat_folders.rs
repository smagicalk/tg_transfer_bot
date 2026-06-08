#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RecommendedChatFolders {
    /// Contains a list of recommended chat folders
    #[serde(rename(
        serialize = "recommendedChatFolders",
        deserialize = "recommendedChatFolders"
    ))]
    RecommendedChatFolders(crate::types::RecommendedChatFolders),
}

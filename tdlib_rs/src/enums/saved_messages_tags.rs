#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedMessagesTags {
    /// Contains a list of tags used in Saved Messages
    #[serde(rename(serialize = "savedMessagesTags", deserialize = "savedMessagesTags"))]
    SavedMessagesTags(crate::types::SavedMessagesTags),
}

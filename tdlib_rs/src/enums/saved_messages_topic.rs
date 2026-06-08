#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedMessagesTopic {
    /// Contains information about a Saved Messages topic
    #[serde(rename(serialize = "savedMessagesTopic", deserialize = "savedMessagesTopic"))]
    SavedMessagesTopic(crate::types::SavedMessagesTopic),
}

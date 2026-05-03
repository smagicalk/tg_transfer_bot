#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallStream {
    /// Describes an available stream in a video chat or a live story
    #[serde(rename(serialize = "groupCallStream", deserialize = "groupCallStream"))]
    GroupCallStream(crate::types::GroupCallStream),
}

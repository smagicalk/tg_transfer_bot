#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPaidMediaType {
    /// The media is a photo. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
    #[serde(rename(
        serialize = "inputPaidMediaTypePhoto",
        deserialize = "inputPaidMediaTypePhoto"
    ))]
    Photo,
    /// The media is a video
    #[serde(rename(
        serialize = "inputPaidMediaTypeVideo",
        deserialize = "inputPaidMediaTypeVideo"
    ))]
    Video(crate::types::InputPaidMediaTypeVideo),
}

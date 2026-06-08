#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaidMedia {
    /// The media is hidden until the invoice is paid
    #[serde(rename(serialize = "paidMediaPreview", deserialize = "paidMediaPreview"))]
    Preview(crate::types::PaidMediaPreview),
    /// The media is a photo
    #[serde(rename(serialize = "paidMediaPhoto", deserialize = "paidMediaPhoto"))]
    Photo(crate::types::PaidMediaPhoto),
    /// The media is a video
    #[serde(rename(serialize = "paidMediaVideo", deserialize = "paidMediaVideo"))]
    Video(crate::types::PaidMediaVideo),
    /// The media is unsupported
    #[serde(rename(
        serialize = "paidMediaUnsupported",
        deserialize = "paidMediaUnsupported"
    ))]
    Unsupported,
}

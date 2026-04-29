#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallVideoQuality {
    /// The worst available video quality
    #[serde(rename(serialize = "groupCallVideoQualityThumbnail", deserialize = "groupCallVideoQualityThumbnail"))]
    Thumbnail,
    /// The medium video quality
    #[serde(rename(serialize = "groupCallVideoQualityMedium", deserialize = "groupCallVideoQualityMedium"))]
    Medium,
    /// The best available video quality
    #[serde(rename(serialize = "groupCallVideoQualityFull", deserialize = "groupCallVideoQualityFull"))]
    Full,
}

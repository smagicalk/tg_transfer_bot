#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MaskPosition {
    /// Position on a photo where a mask is placed
    #[serde(rename(serialize = "maskPosition", deserialize = "maskPosition"))]
    MaskPosition(crate::types::MaskPosition),
}

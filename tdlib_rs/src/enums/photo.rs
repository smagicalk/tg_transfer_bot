#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Photo {
    /// Describes a photo
    #[serde(rename(serialize = "photo", deserialize = "photo"))]
    Photo(crate::types::Photo),
}

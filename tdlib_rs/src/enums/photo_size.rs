#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PhotoSize {
    /// Describes an image in JPEG format
    #[serde(rename(serialize = "photoSize", deserialize = "photoSize"))]
    PhotoSize(crate::types::PhotoSize),
}

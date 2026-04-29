#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Background {
    /// Describes a chat background
    #[serde(rename(serialize = "background", deserialize = "background"))]
    Background(crate::types::Background),
}

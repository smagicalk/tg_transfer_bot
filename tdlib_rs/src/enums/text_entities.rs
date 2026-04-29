#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextEntities {
    /// Contains a list of text entities
    #[serde(rename(serialize = "textEntities", deserialize = "textEntities"))]
    TextEntities(crate::types::TextEntities),
}

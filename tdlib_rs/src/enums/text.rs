#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Text {
    /// Contains some text
    #[serde(rename(serialize = "text", deserialize = "text"))]
    Text(crate::types::Text),
}

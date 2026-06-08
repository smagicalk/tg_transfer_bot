#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Stories {
    /// Represents a list of stories
    #[serde(rename(serialize = "stories", deserialize = "stories"))]
    Stories(crate::types::Stories),
}

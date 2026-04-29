#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarCount {
    /// Contains a number of Telegram Stars
    #[serde(rename(serialize = "starCount", deserialize = "starCount"))]
    StarCount(crate::types::StarCount),
}

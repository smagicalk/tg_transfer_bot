#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Poll {
    /// Describes a poll
    #[serde(rename(serialize = "poll", deserialize = "poll"))]
    Poll(crate::types::Poll),
}

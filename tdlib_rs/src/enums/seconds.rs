#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Seconds {
    /// Contains a value representing a number of seconds
    #[serde(rename(serialize = "seconds", deserialize = "seconds"))]
    Seconds(crate::types::Seconds),
}

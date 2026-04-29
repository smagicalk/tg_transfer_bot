#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Data {
    /// Contains some binary data
    #[serde(rename(serialize = "data", deserialize = "data"))]
    Data(crate::types::Data),
}

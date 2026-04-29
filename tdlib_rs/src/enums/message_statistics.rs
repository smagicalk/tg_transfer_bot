#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageStatistics {
    /// A detailed statistics about a message
    #[serde(rename(serialize = "messageStatistics", deserialize = "messageStatistics"))]
    MessageStatistics(crate::types::MessageStatistics),
}

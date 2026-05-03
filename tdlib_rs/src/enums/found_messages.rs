#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundMessages {
    /// Contains a list of messages found by a search
    #[serde(rename(serialize = "foundMessages", deserialize = "foundMessages"))]
    FoundMessages(crate::types::FoundMessages),
}

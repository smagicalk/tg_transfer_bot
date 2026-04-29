#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DraftMessage {
    /// Contains information about a message draft
    #[serde(rename(serialize = "draftMessage", deserialize = "draftMessage"))]
    DraftMessage(crate::types::DraftMessage),
}

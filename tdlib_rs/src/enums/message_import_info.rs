#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageImportInfo {
    /// Contains information about a message created with importMessages
    #[serde(rename(serialize = "messageImportInfo", deserialize = "messageImportInfo"))]
    MessageImportInfo(crate::types::MessageImportInfo),
}

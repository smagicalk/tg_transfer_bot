#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageViewer {
    /// Represents a viewer of a message
    #[serde(rename(serialize = "messageViewer", deserialize = "messageViewer"))]
    MessageViewer(crate::types::MessageViewer),
}

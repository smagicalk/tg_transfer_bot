#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AttachmentMenuBot {
    /// Represents a bot, which can be added to attachment or side menu
    #[serde(rename(serialize = "attachmentMenuBot", deserialize = "attachmentMenuBot"))]
    AttachmentMenuBot(crate::types::AttachmentMenuBot),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AttachmentMenuBotColor {
    /// Describes a color to highlight a bot added to attachment menu
    #[serde(rename(serialize = "attachmentMenuBotColor", deserialize = "attachmentMenuBotColor"))]
    AttachmentMenuBotColor(crate::types::AttachmentMenuBotColor),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat needs to be open with the provided internal link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TargetChatInternalLink {
    /// An internal link pointing to the chat
    pub link: crate::enums::InternalLinkType,
}

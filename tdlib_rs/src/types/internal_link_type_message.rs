#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a Telegram message or a forum topic. Call getMessageLinkInfo with the given URL to process the link,
/// and then open received forum topic or chat and show the message there
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeMessage {
    /// URL to be passed to getMessageLinkInfo
    pub url: String,
}

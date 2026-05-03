#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to boost a Telegram chat. Call getChatBoostLinkInfo with the given URL to process the link.
/// If the chat is found, then call getChatBoostStatus and getAvailableChatBoostSlots to get the current boost status and check whether the chat can be boosted.
/// If the user wants to boost the chat and the chat can be boosted, then call boostChat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatBoost {
    /// URL to be passed to getChatBoostLinkInfo
    pub url: String,
}

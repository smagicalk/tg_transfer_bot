#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link.
/// If the chat is found, open its profile information screen or the chat itself.
/// If draft text isn't empty and the chat is a private chat with a regular user, then put the draft text in the input field
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePublicChat {
    /// Username of the chat
    pub chat_username: String,
    /// Draft text for message to send in the chat
    pub draft_text: String,
    /// True, if chat profile information screen must be opened; otherwise, the chat itself must be opened
    pub open_profile: bool,
}

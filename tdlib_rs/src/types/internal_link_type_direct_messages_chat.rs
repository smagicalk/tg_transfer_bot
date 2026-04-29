#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a channel direct messages chat by username of the channel. Call searchPublicChat with the given chat username to process the link.
/// If the chat is found and is channel, open the direct messages chat of the channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeDirectMessagesChat {
    /// Username of the channel
    pub channel_username: String,
}

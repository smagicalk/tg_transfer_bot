#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat is a private chat with an administrator of a chat to which the user sent join request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarJoinRequest {
    /// Title of the chat to which the join request was sent
    pub title: String,
    /// True, if the join request was sent to a channel chat
    pub is_channel: bool,
    /// Point in time (Unix timestamp) when the join request was sent
    pub request_date: i32,
}

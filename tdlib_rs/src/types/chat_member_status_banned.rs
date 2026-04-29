#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user or the chat was banned (and hence is not a member of the chat). Implies the user can't return to the chat, view messages, or be used as a participant identifier to join a video chat of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusBanned {
    /// Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever. Always 0 in basic groups
    pub banned_until_date: i32,
}

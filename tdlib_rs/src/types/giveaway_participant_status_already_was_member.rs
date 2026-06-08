#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user can't participate in the giveaway, because they have already been member of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParticipantStatusAlreadyWasMember {
    /// Point in time (Unix timestamp) when the user joined the chat
    pub joined_chat_date: i32,
}

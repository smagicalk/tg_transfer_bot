#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user can't participate in the giveaway, because they are an administrator in one of the chats that created the giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParticipantStatusAdministrator {
    /// Identifier of the chat administered by the user
    pub chat_id: i64,
}

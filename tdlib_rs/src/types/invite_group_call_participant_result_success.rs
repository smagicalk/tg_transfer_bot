#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user was invited and a service message of the type messageGroupCall was sent which can be used in declineGroupCallInvitation to cancel the invitation
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InviteGroupCallParticipantResultSuccess {
    /// Identifier of the chat with the invitation message
    pub chat_id: i64,
    /// Identifier of the message
    pub message_id: i64,
}

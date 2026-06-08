#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat's has_scheduled_messages field has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatHasScheduledMessages {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of has_scheduled_messages
    pub has_scheduled_messages: bool,
}

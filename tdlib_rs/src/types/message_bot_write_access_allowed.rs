#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user allowed the bot to send messages
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageBotWriteAccessAllowed {
    /// The reason why the bot was allowed to write messages
    pub reason: crate::enums::BotWriteAccessAllowReason,
}

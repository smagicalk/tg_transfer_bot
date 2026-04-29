#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes settings for messages that are automatically sent by a Telegram Business account when it is away
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessAwayMessageSettings {
    /// Unique quick reply shortcut identifier for the away messages
    pub shortcut_id: i32,
    /// Chosen recipients of the away messages
    pub recipients: crate::types::BusinessRecipients,
    /// Settings used to check whether the current user is away
    pub schedule: crate::enums::BusinessAwayMessageSchedule,
    /// True, if the messages must not be sent if the account was online in the last 10 minutes
    pub offline_only: bool,
}

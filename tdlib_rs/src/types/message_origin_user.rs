#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message was originally sent by a known user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginUser {
    /// Identifier of the user who originally sent the message
    pub sender_user_id: i64,
}

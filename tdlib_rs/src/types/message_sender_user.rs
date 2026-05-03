#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message was sent by a known user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSenderUser {
    /// Identifier of the user who sent the message
    pub user_id: i64,
}

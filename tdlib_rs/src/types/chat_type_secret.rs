#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A secret chat with a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatTypeSecret {
    /// Secret chat identifier
    pub secret_chat_id: i32,
    /// User identifier of the other user in the secret chat
    pub user_id: i64,
}

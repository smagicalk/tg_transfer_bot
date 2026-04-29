#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Chat permissions were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatPermissions {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat permissions
    pub permissions: crate::types::ChatPermissions,
}

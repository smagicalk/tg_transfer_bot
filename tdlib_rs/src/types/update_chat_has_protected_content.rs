#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat content was allowed or restricted for saving
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatHasProtectedContent {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of has_protected_content
    pub has_protected_content: bool,
}

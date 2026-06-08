#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat background was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatBackground {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat background; may be null if background was reset to default
    pub background: Option<crate::types::ChatBackground>,
}

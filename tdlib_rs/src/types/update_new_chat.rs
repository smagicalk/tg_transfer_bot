#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewChat {
    /// The chat
    pub chat: crate::types::Chat,
}

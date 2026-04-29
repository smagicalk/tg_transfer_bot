#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat needs to be chosen by the user among chats of the specified types
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TargetChatChosen {
    /// Allowed types for the chat
    pub types: crate::types::TargetChatTypes,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat invite link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TmeUrlTypeChatInvite {
    /// Information about the chat invite link
    pub info: crate::types::ChatInviteLinkInfo,
}

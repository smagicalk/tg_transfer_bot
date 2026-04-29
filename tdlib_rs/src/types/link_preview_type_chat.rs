#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeChat {
    /// Type of the chat
    pub r#type: crate::enums::InviteLinkChatType,
    /// Photo of the chat; may be null
    pub photo: Option<crate::types::ChatPhoto>,
    /// True, if the link only creates join request
    pub creates_join_request: bool,
}

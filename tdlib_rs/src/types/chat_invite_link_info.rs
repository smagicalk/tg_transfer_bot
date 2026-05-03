#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a chat invite link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkInfo {
    /// Chat identifier of the invite link; 0 if the user has no access to the chat before joining
    pub chat_id: i64,
    /// If non-zero, the amount of time for which read access to the chat will remain available, in seconds
    pub accessible_for: i32,
    /// Type of the chat
    pub r#type: crate::enums::InviteLinkChatType,
    /// Title of the chat
    pub title: String,
    /// Chat photo; may be null
    pub photo: Option<crate::types::ChatPhotoInfo>,
    /// Identifier of the accent color for chat title and background of chat photo
    pub accent_color_id: i32,
    /// Chat description
    pub description: String,
    /// Number of members in the chat
    pub member_count: i32,
    /// User identifiers of some chat members that may be known to the current user
    pub member_user_ids: Vec<i64>,
    /// Information about subscription plan that must be paid by the user to use the link; may be null if the link doesn't require subscription
    pub subscription_info: Option<crate::types::ChatInviteLinkSubscriptionInfo>,
    /// True, if the link only creates join request
    pub creates_join_request: bool,
    /// True, if the chat is a public supergroup or channel, i.e. it has a username or it is a location-based supergroup
    pub is_public: bool,
    /// Information about verification status of the chat; may be null if none
    pub verification_status: Option<crate::types::VerificationStatus>,
}

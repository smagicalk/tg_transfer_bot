#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a chat invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLink {
    /// Chat invite link
    pub invite_link: String,
    /// Name of the link
    pub name: String,
    /// User identifier of an administrator created the link
    pub creator_user_id: i64,
    /// Point in time (Unix timestamp) when the link was created
    pub date: i32,
    /// Point in time (Unix timestamp) when the link was last edited; 0 if never or unknown
    pub edit_date: i32,
    /// Point in time (Unix timestamp) when the link will expire; 0 if never
    pub expiration_date: i32,
    /// Information about subscription plan that is applied to the users joining the chat by the link; may be null if the link doesn't require subscription
    pub subscription_pricing: Option<crate::types::StarSubscriptionPricing>,
    /// The maximum number of members, which can join the chat using the link simultaneously; 0 if not limited. Always 0 if the link requires approval
    pub member_limit: i32,
    /// Number of chat members, which joined the chat using the link
    pub member_count: i32,
    /// Number of chat members, which joined the chat using the link, but have already left because of expired subscription; for subscription links only
    pub expired_member_count: i32,
    /// Number of pending join requests created using this link
    pub pending_join_request_count: i32,
    /// True, if the link only creates join request. If true, total number of joining members will be unlimited
    pub creates_join_request: bool,
    /// True, if the link is primary. Primary invite link can't have name, expiration date, or usage limit. There is exactly one primary invite link for each administrator with can_invite_users right at a given time
    pub is_primary: bool,
    /// True, if the link was revoked
    pub is_revoked: bool,
}

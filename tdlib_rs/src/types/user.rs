#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a user
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    /// User identifier
    pub id: i64,
    /// First name of the user
    pub first_name: String,
    /// Last name of the user
    pub last_name: String,
    /// Usernames of the user; may be null
    pub usernames: Option<crate::types::Usernames>,
    /// Phone number of the user
    pub phone_number: String,
    /// Current online status of the user
    pub status: crate::enums::UserStatus,
    /// Profile photo of the user; may be null
    pub profile_photo: Option<crate::types::ProfilePhoto>,
    /// Identifier of the accent color for name, and backgrounds of profile photo, reply header, and link preview
    pub accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub background_custom_emoji_id: i64,
    /// Color scheme based on an upgraded gift to be used for the user instead of accent_color_id and background_custom_emoji_id; may be null if none
    pub upgraded_gift_colors: Option<crate::types::UpgradedGiftColors>,
    /// Identifier of the accent color for the user's profile; -1 if none
    pub profile_accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the background of the user's profile; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub profile_background_custom_emoji_id: i64,
    /// Emoji status to be shown instead of the default Telegram Premium badge; may be null
    pub emoji_status: Option<crate::types::EmojiStatus>,
    /// The user is a contact of the current user
    pub is_contact: bool,
    /// The user is a contact of the current user and the current user is a contact of the user
    pub is_mutual_contact: bool,
    /// The user is a close friend of the current user; implies that the user is a contact
    pub is_close_friend: bool,
    /// Information about verification status of the user; may be null if none
    pub verification_status: Option<crate::types::VerificationStatus>,
    /// True, if the user is a Telegram Premium user
    pub is_premium: bool,
    /// True, if the user is Telegram support account
    pub is_support: bool,
    /// Information about restrictions that must be applied to the corresponding private chat; may be null if none
    pub restriction_info: Option<crate::types::RestrictionInfo>,
    /// State of active stories of the user; may be null if the user has no active stories
    pub active_story_state: Option<crate::enums::ActiveStoryState>,
    /// True, if the user may restrict new chats with non-contacts. Use canSendMessageToUser to check whether the current user can message the user or try to create a chat with them
    pub restricts_new_chats: bool,
    /// Number of Telegram Stars that must be paid by general user for each sent message to the user. If positive and userFullInfo is unknown, use canSendMessageToUser to check whether the current user must pay
    pub paid_message_star_count: i64,
    /// If false, the user is inaccessible, and the only information known about the user is inside this class. Identifier of the user can't be passed to any method
    pub have_access: bool,
    /// Type of the user
    pub r#type: crate::enums::UserType,
    /// IETF language tag of the user's language; only available to bots
    pub language_code: String,
    /// True, if the user added the current bot to attachment menu; only available to bots
    pub added_to_attachment_menu: bool,
}

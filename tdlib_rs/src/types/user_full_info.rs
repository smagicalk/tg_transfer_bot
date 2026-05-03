#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains full information about a user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserFullInfo {
    /// User profile photo set by the current user for the contact; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown.
    /// If non-null, then it is the same photo as in user.profile_photo and chat.photo. This photo isn't returned in the list of user photos
    pub personal_photo: Option<crate::types::ChatPhoto>,
    /// User profile photo; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown.
    /// If non-null and personal_photo is null, then it is the same photo as in user.profile_photo and chat.photo
    pub photo: Option<crate::types::ChatPhoto>,
    /// User profile photo visible if the main photo is hidden by privacy settings; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown.
    /// If non-null and both photo and personal_photo are null, then it is the same photo as in user.profile_photo and chat.photo. This photo isn't returned in the list of user photos
    pub public_photo: Option<crate::types::ChatPhoto>,
    /// Block list to which the user is added; may be null if none
    pub block_list: Option<crate::enums::BlockList>,
    /// True, if the user can be called
    pub can_be_called: bool,
    /// True, if a video call can be created with the user
    pub supports_video_calls: bool,
    /// True, if the user can't be called due to their privacy settings
    pub has_private_calls: bool,
    /// True, if the user can't be linked in forwarded messages due to their privacy settings
    pub has_private_forwards: bool,
    /// True, if voice and video notes can't be sent or forwarded to the user
    pub has_restricted_voice_and_video_note_messages: bool,
    /// True, if the user has posted to profile stories
    pub has_posted_to_profile_stories: bool,
    /// True, if the user always enabled sponsored messages; known only for the current user
    pub has_sponsored_messages_enabled: bool,
    /// True, if the current user needs to explicitly allow to share their phone number with the user when the method addContact is used
    pub need_phone_number_privacy_exception: bool,
    /// True, if the user set chat background for both chat users and it wasn't reverted yet
    pub set_chat_background: bool,
    /// A short user bio; may be null for bots
    pub bio: Option<crate::types::FormattedText>,
    /// Birthdate of the user; may be null if unknown
    pub birthdate: Option<crate::types::Birthdate>,
    /// Identifier of the personal chat of the user; 0 if none
    pub personal_chat_id: i64,
    /// Number of saved to profile gifts for other users or the total number of received gifts for the current user
    pub gift_count: i32,
    /// Number of group chats where both the other user and the current user are a member; 0 for the current user
    pub group_in_common_count: i32,
    /// Number of Telegram Stars that must be paid by the user for each sent message to the current user
    pub incoming_paid_message_star_count: i64,
    /// Number of Telegram Stars that must be paid by the current user for each sent message to the user
    pub outgoing_paid_message_star_count: i64,
    /// Settings for gift receiving for the user
    pub gift_settings: crate::types::GiftSettings,
    /// Information about verification status of the user provided by a bot; may be null if none or unknown
    pub bot_verification: Option<crate::types::BotVerification>,
    /// The main tab chosen by the user; may be null if not chosen manually
    pub main_profile_tab: Option<crate::enums::ProfileTab>,
    /// The first audio file added to the user's profile; may be null if none
    pub first_profile_audio: Option<crate::types::Audio>,
    /// The current rating of the user; may be null if none
    pub rating: Option<crate::types::UserRating>,
    /// The rating of the user after the next change; may be null if the user isn't the current user or there are no pending rating changes
    pub pending_rating: Option<crate::types::UserRating>,
    /// Unix timestamp when rating of the user will change to pending_rating; 0 if the user isn't the current user or there are no pending rating changes
    pub pending_rating_date: i32,
    /// Note added to the user's contact; may be null if none
    pub note: Option<crate::types::FormattedText>,
    /// Information about business settings for Telegram Business accounts; may be null if none
    pub business_info: Option<crate::types::BusinessInfo>,
    /// For bots, information about the bot; may be null if the user isn't a bot
    pub bot_info: Option<crate::types::BotInfo>,
}

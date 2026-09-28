//!
//! TDLib `user` domain types.
//!
//! Types, enums, and functions for user accounts, privacy settings, and contacts.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a contact of a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Contact {
    /// Phone number of the user
    pub phone_number: String,
    /// First name of the user; 1-64 characters
    pub first_name: String,
    /// Last name of the user; 0-64 characters
    pub last_name: String,
    /// Additional data about the user in a form of vCard; 0-2048 bytes in length
    pub vcard: String,
    /// Identifier of the user, if known; 0 otherwise
    pub user_id: i64,
}

/// A regular user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserTypeRegular {
}

/// A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserTypeDeleted {
}

/// No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserTypeUnknown {
}

/// Describes a user who had or will have a birthday soon
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CloseBirthdayUser {
    /// User identifier
    pub user_id: i64,
    /// Birthdate of the user
    pub birthdate: crate::types::Birthdate,
}

/// Describes a bid of the current user in an auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserAuctionBid {
    /// The number of Telegram Stars that were put in the bid
    pub star_count: i64,
    /// Point in time (Unix timestamp) when the bid was made
    pub bid_date: i32,
    /// The minimum number of Telegram Stars that can be put for the next bid
    pub next_bid_star_count: i64,
    /// Identifier of the user or the chat that will receive the auctioned item. If the auction is opened in context of another user or chat, then a warning is supposed to be shown to the current user
    pub owner_id: crate::enums::MessageSender,
    /// True, if the bid was returned to the user, because it was outbid and can't win anymore
    pub was_returned: bool,
}

/// The transaction is a deposit of Telegram Stars by another user; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeUserDeposit {
    /// Identifier of the user who gifted Telegram Stars; 0 if the user was anonymous
    pub user_id: i64,
    /// The sticker to be shown in the transaction information; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// Contains description of user rating
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserRating {
    /// The level of the user; may be negative
    pub level: i32,
    /// True, if the maximum level is reached
    pub is_maximum_level_reached: bool,
    /// Numerical value of the rating
    pub rating: i64,
    /// The rating required for the current level
    pub current_level_rating: i64,
    /// The rating required for the next level; 0 if the maximum level is reached
    pub next_level_rating: i64,
}

/// Describes usernames assigned to a user, a supergroup, or a channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Usernames {
    /// List of active usernames; the first one must be shown as the primary username. The order of active usernames can be changed with reorderActiveUsernames, reorderBotActiveUsernames or reorderSupergroupActiveUsernames
    pub active_usernames: Vec<String>,
    /// List of currently disabled usernames; the username can be activated with toggleUsernameIsActive, toggleBotUsernameIsActive, or toggleSupergroupUsernameIsActive
    pub disabled_usernames: Vec<String>,
    /// Active or disabled username, which may be changed with setUsername or setSupergroupUsername
    pub editable_username: String,
    /// Collectible usernames that were purchased at https:fragment.com and can be passed to getCollectibleItemInfo for more details
    pub collectible_usernames: Vec<String>,
}

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
    #[serde(rename = "type")]
    pub r#type: crate::enums::UserType,
    /// IETF language tag of the user's language; only available to bots
    pub language_code: String,
    /// True, if the user added the current bot to attachment menu; only available to bots
    pub added_to_attachment_menu: bool,
}

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
    /// Identifier of the community to which chat with the bot was added; for bots only
    pub community_id: i64,
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
    /// True, if the user uses an unofficial application that poses a security risk
    pub uses_unofficial_app: bool,
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

/// Represents a list of users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Users {
    /// Approximate total number of users found
    pub total_count: i32,
    /// A list of user identifiers
    pub user_ids: Vec<i64>,
}

/// Represents a list of found users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundUsers {
    /// Identifiers of the found users
    pub user_ids: Vec<i64>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// A button that requests users to be shared by the current user; available only in private chats. Use the method shareUsersWithBot to complete the request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestUsers {
    /// Unique button identifier
    pub id: i32,
    /// True, if the shared users must or must not be bots
    pub restrict_user_is_bot: bool,
    /// True, if the shared users must be bots; otherwise, the shared users must not be bots. Ignored if restrict_user_is_bot is false
    pub user_is_bot: bool,
    /// True, if the shared users must or must not be Telegram Premium users
    pub restrict_user_is_premium: bool,
    /// True, if the shared users must be Telegram Premium users; otherwise, the shared users must not be Telegram Premium users. Ignored if restrict_user_is_premium is false
    pub user_is_premium: bool,
    /// The maximum number of users to share
    pub max_quantity: i32,
    /// Pass true to request name of the users; bots only
    pub request_name: bool,
    /// Pass true to request username of the users; bots only
    pub request_username: bool,
    /// Pass true to request photo of the users; bots only
    pub request_photo: bool,
}

/// A button with a user reference to be handled in the same way as textEntityTypeMentionName entities
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeUser {
    /// User identifier
    pub user_id: i64,
}

/// Contains information about a user shared with a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SharedUser {
    /// User identifier
    pub user_id: i64,
    /// First name of the user; for bots only
    pub first_name: String,
    /// Last name of the user; for bots only
    pub last_name: String,
    /// Username of the user; for bots only
    pub username: String,
    /// Profile photo of the user; for bots only; may be null
    pub photo: Option<crate::types::Photo>,
}

/// The link is a link to a user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeUser {
    /// Photo of the user; may be null if none
    pub photo: Option<crate::types::ChatPhoto>,
    /// True, if the user is a bot
    pub is_bot: bool,
}

/// A username
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CollectibleItemTypeUsername {
    /// The username
    pub username: String,
}

/// The user's status has never been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusEmpty {
}

/// The user is online
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusOnline {
    /// Point in time (Unix timestamp) when the user's online status will expire
    pub expires: i32,
}

/// The user is offline
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusOffline {
    /// Point in time (Unix timestamp) when the user was last online
    pub was_online: i32,
}

/// The user was online recently
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusRecently {
    /// Exact user's status is hidden because the current user enabled userPrivacySettingShowStatus privacy setting for the user and has no Telegram Premium
    pub by_my_privacy_settings: bool,
}

/// The user is offline, but was online last week
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusLastWeek {
    /// Exact user's status is hidden because the current user enabled userPrivacySettingShowStatus privacy setting for the user and has no Telegram Premium
    pub by_my_privacy_settings: bool,
}

/// The user is offline, but was online last month
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusLastMonth {
    /// Exact user's status is hidden because the current user enabled userPrivacySettingShowStatus privacy setting for the user and has no Telegram Premium
    pub by_my_privacy_settings: bool,
}

/// The user requested to resend the code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResendCodeReasonUserRequest {
}

/// Describes a contact to import
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ImportedContact {
    /// Phone number of the user
    pub phone_number: String,
    /// First name of the user; 1-64 characters
    pub first_name: String,
    /// Last name of the user; 0-64 characters
    pub last_name: String,
    /// Note to add about the user; 0-getOption("user_note_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed;
    /// pass null to keep the current user's note
    pub note: crate::types::FormattedText,
}

/// Represents the result of an importContacts request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ImportedContacts {
    /// User identifiers of the imported contacts in the same order as they were specified in the request; 0 if the contact is not yet a registered user
    pub user_ids: Vec<i64>,
    /// The number of users that imported the corresponding contact; 0 for already registered users or if unavailable
    pub importer_count: Vec<i32>,
}

/// Contains an HTTPS URL, which can be used to get information about a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserLink {
    /// The URL
    pub url: String,
    /// Left time for which the link is valid, in seconds; 0 if the link is a public username link
    pub expires_in: i32,
}

/// Represents a user contact
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultContact {
    /// Unique identifier of the query result
    pub id: String,
    /// User contact
    pub contact: crate::types::Contact,
    /// URL of the result thumbnail, if it exists
    pub thumbnail_url: String,
    /// Thumbnail width, if known
    pub thumbnail_width: i32,
    /// Thumbnail height, if known
    pub thumbnail_height: i32,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageRichMessage, inputMessageInvoice, inputMessageLiveLocation, inputMessageLocation, inputMessageVenue or inputMessageContact
    pub input_message_content: crate::enums::InputMessageContent,
}

/// Represents a user contact
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultContact {
    /// Unique identifier of the query result
    pub id: String,
    /// A user contact
    pub contact: crate::types::Contact,
    /// Result thumbnail in JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
}

/// A rule to allow all users to do something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleAllowAll {
}

/// A rule to allow all contacts of the user to do something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleAllowContacts {
}

/// A rule to allow certain specified users to do something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleAllowUsers {
    /// The user identifiers, total number of users in all rules must not exceed 1000
    pub user_ids: Vec<i64>,
}

/// A rule to restrict all users from doing something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleRestrictAll {
}

/// A rule to restrict all contacts of the user from doing something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleRestrictContacts {
}

/// A rule to restrict all specified users from doing something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleRestrictUsers {
    /// The user identifiers, total number of users in all rules must not exceed 1000
    pub user_ids: Vec<i64>,
}

/// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRules {
    /// A list of rules
    pub rules: Vec<crate::enums::UserPrivacySettingRule>,
}

/// A privacy setting for managing whether the user's online status is visible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingShowStatus {
}

/// A privacy setting for managing whether the user's phone number is visible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingShowPhoneNumber {
}

/// A privacy setting for managing whether the user's bio is visible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingShowBio {
}

/// A privacy setting for managing whether the user's birthdate is visible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingShowBirthdate {
}

/// A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to "Allow contacts" or "Allow all"
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingAllowFindingByPhoneNumber {
}

/// A privacy setting for managing whether received gifts are automatically shown on the user's profile page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingAutosaveGifts {
}

/// The link is a link to the Contacts tab or page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeContactsPage {
    /// Section of the page; may be one of
    /// "", "search", "sort", "new", "invite", "manage"
    pub section: String,
}

/// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link.
/// If the user is found, then call createPrivateChat and open user's profile information screen or the chat itself. If draft text isn't empty, then put the draft text in the input field
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeUserPhoneNumber {
    /// Phone number of the user
    pub phone_number: String,
    /// Draft text for message to send in the chat
    pub draft_text: String,
    /// True, if user's profile information screen must be opened; otherwise, the chat itself must be opened
    pub open_profile: bool,
}

/// The link is a link to a user by a temporary token. Call searchUserByToken with the given token to process the link.
/// If the user is found, then call createPrivateChat and open the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeUserToken {
    /// The token
    pub token: String,
}

/// A URL linking to a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TmeUrlTypeUser {
    /// Identifier of the user
    pub user_id: i64,
}

/// The user went online or offline
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUserStatus {
    /// User identifier
    pub user_id: i64,
    /// New status of the user
    pub status: crate::enums::UserStatus,
}

/// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUser {
    /// New data about the user
    pub user: crate::types::User,
}

/// Some data in userFullInfo has been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUserFullInfo {
    /// User identifier
    pub user_id: i64,
    /// New full information about the user
    pub user_full_info: crate::types::UserFullInfo,
}

/// Some privacy setting rules have been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUserPrivacySettingRules {
    /// The privacy setting
    pub setting: crate::enums::UserPrivacySetting,
    /// New privacy rules
    pub rules: crate::types::UserPrivacySettingRules,
}

/// The list of contacts that had birthdays recently or will have birthday soon has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateContactCloseBirthdays {
    /// List of contact users with close birthday
    pub close_birthday_users: Vec<crate::types::CloseBirthdayUser>,
}

/// Subscription of a user to the bot was changed; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateUserSubscription {
    /// Identifier of the user
    pub user_id: i64,
    /// Bot-specified subscription invoice payload
    pub payload: String,
    /// True, if the subscription was canceled
    pub is_canceled: bool,
    /// True, if the subscription was restored
    pub is_restored: bool,
    /// True, if the payment for the subscription has failed
    pub is_payment_failed: bool,
}

/// Contains custom information about the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserSupportInfo {
    /// Information message
    pub message: crate::types::FormattedText,
    /// Information author
    pub author: String,
    /// Information change date
    pub date: i32,
}


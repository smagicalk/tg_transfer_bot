//!
//! TDLib `bot` domain types.
//!
//! Types, enums, and functions for Telegram Bots, Web Apps, and inline queries.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A bot (see https:core.telegram.org/bots)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserTypeBot {
    /// True, if the bot is owned by the current user and can be edited using the methods toggleBotUsernameIsActive, reorderBotActiveUsernames, setBotProfilePhoto, setBotName, setBotInfoDescription, and setBotInfoShortDescription
    pub can_be_edited: bool,
    /// True, if the bot can be invited to basic group and supergroup chats
    pub can_join_groups: bool,
    /// True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages
    pub can_read_all_group_messages: bool,
    /// True, if the bot has the main Web App
    pub has_main_web_app: bool,
    /// True, if the bot has topics
    pub has_topics: bool,
    /// True, if users can create and delete topics in the chat with the bot
    pub allows_users_to_create_topics: bool,
    /// True, if the bot can manage other bots
    pub can_manage_bots: bool,
    /// True, if the bot supports inline queries
    pub is_inline: bool,
    /// Placeholder for inline queries (displayed on the application input field)
    pub inline_query_placeholder: String,
    /// True, if the bot can be queried by username from any non-secret chat
    pub supports_guest_queries: bool,
    /// True, if the bot can be set as a guard bot in supergroup chats
    pub is_guard: bool,
    /// True, if the location of the user is expected to be sent with every inline query to this bot
    pub need_location: bool,
    /// True, if the bot supports connection to user accounts for chat automation
    pub can_connect_to_business: bool,
    /// True, if the bot can be added to attachment or side menu
    pub can_be_added_to_attachment_menu: bool,
    /// The number of recently active users of the bot
    pub active_user_count: i32,
}

/// Represents a command supported by a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommand {
    /// Text of the bot command
    pub command: String,
    /// Description of the bot command
    pub description: String,
    /// True, if the command must send an ephemeral message instead of a regular one
    pub is_ephemeral: bool,
}

/// Contains a list of bot commands
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommands {
    /// Bot's user identifier
    pub bot_user_id: i64,
    /// List of bot commands
    pub commands: Vec<crate::types::BotCommand>,
}

/// Describes a button to be shown instead of bot commands menu button
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotMenuButton {
    /// Text of the button
    pub text: String,
    /// URL of a Web App to open when the button is pressed. If the link is of the type internalLinkTypeWebApp, then it must be processed accordingly. Otherwise, the link must be passed to openWebApp
    pub url: String,
}

/// Describes users that have access to a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotAccessSettings {
    /// True, if access to the bot is restricted to its owner and selected users
    pub is_restricted: bool,
    /// Identifiers of the users who can use the bot additionally to the owner of the bot
    pub added_user_ids: Vec<i64>,
}

/// Describes parameters of verification that is provided by a bot
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotVerificationParameters {
    /// Identifier of the custom emoji that is used as the verification sign
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
    /// Name of the organization that provides verification
    pub organization_name: String,
    /// Default custom description of verification reason to be used as placeholder in setMessageSenderBotVerification; may be null if none
    pub default_custom_description: Option<crate::types::FormattedText>,
    /// True, if the bot is allowed to provide custom description for verified entities
    pub can_set_custom_description: bool,
}

/// Describes verification status provided by a bot
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotVerification {
    /// Identifier of the bot that provided the verification
    pub bot_user_id: i64,
    /// Identifier of the custom emoji that is used as the verification sign
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
    /// Custom description of verification reason set by the bot. Can contain only Mention, Hashtag, Cashtag, PhoneNumber, BankCardNumber, Url, and EmailAddress entities
    pub custom_description: crate::types::FormattedText,
}

/// The transaction is a purchase of paid media from a bot or a business account by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotPaidMediaPurchase {
    /// Identifier of the bot or the business account user who sent the paid media
    pub user_id: i64,
    /// The bought media if the transaction wasn't refunded
    pub media: Vec<crate::enums::PaidMedia>,
}

/// The transaction is a sale of paid media by the bot or a business account managed by the bot; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotPaidMediaSale {
    /// Identifier of the user who bought the media
    pub user_id: i64,
    /// The bought media
    pub media: Vec<crate::enums::PaidMedia>,
    /// Bot-provided payload
    pub payload: String,
    /// Information about the affiliate which received commission from the transaction; may be null if none
    pub affiliate: Option<crate::types::AffiliateInfo>,
}

/// The transaction is a purchase of a subscription from a bot or a business account by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotSubscriptionPurchase {
    /// Identifier of the bot or the business account user who created the subscription link
    pub user_id: i64,
    /// The number of seconds between consecutive Telegram Star debitings
    pub subscription_period: i32,
    /// Information about the bought subscription
    pub product_info: crate::types::ProductInfo,
}

/// The transaction is a sale of a subscription by the bot; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBotSubscriptionSale {
    /// Identifier of the user who bought the subscription
    pub user_id: i64,
    /// The number of seconds between consecutive Telegram Star debitings
    pub subscription_period: i32,
    /// Information about the bought subscription
    pub product_info: crate::types::ProductInfo,
    /// Invoice payload
    pub invoice_payload: String,
    /// Information about the affiliate which received commission from the transaction; may be null if none
    pub affiliate: Option<crate::types::AffiliateInfo>,
}

/// Contains information about a bot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BotInfo {
    /// The text that is shown on the bot's profile page and is sent together with the link when users share the bot
    pub short_description: String,
    /// The text shown in the chat with the bot if the chat is empty
    pub description: String,
    /// Photo shown in the chat with the bot if the chat is empty; may be null
    pub photo: Option<crate::types::Photo>,
    /// Animation shown in the chat with the bot if the chat is empty; may be null
    pub animation: Option<crate::types::Animation>,
    /// Identifier of the bot, which manages the bot; 0 if none or unknown; for owner of the bot only
    pub manager_bot_user_id: i64,
    /// Information about a button to show instead of the bot commands menu button; may be null if ordinary bot commands menu must be shown
    pub menu_button: Option<crate::types::BotMenuButton>,
    /// List of the bot commands
    pub commands: Vec<crate::types::BotCommand>,
    /// The HTTP link to the privacy policy of the bot. If empty, then /privacy command must be used if supported by the bot. If the command isn't supported, then https:telegram.org/privacy-tpa must be opened
    pub privacy_policy_url: String,
    /// Default administrator rights for adding the bot to basic group and supergroup chats; may be null
    pub default_group_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// Default administrator rights for adding the bot to channels; may be null
    pub default_channel_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// Information about the affiliate program of the bot; may be null if none
    pub affiliate_program: Option<crate::types::AffiliateProgramInfo>,
    /// Default light background color for bot Web Apps; -1 if not specified
    pub web_app_background_light_color: i32,
    /// Default dark background color for bot Web Apps; -1 if not specified
    pub web_app_background_dark_color: i32,
    /// Default light header color for bot Web Apps; -1 if not specified
    pub web_app_header_light_color: i32,
    /// Default dark header color for bot Web Apps; -1 if not specified
    pub web_app_header_dark_color: i32,
    /// Parameters of the verification that can be provided by the bot; may be null if none or the current user isn't the owner of the bot
    pub verification_parameters: Option<crate::types::BotVerificationParameters>,
    /// True, if the bot's revenue statistics are available to the current user
    pub can_get_revenue_statistics: bool,
    /// True, if the bot can manage emoji status of the current user
    pub can_manage_emoji_status: bool,
    /// True, if the bot has media previews
    pub has_media_previews: bool,
    /// The internal link, which can be used to edit bot commands; may be null
    pub edit_commands_link: Option<crate::enums::InternalLinkType>,
    /// The internal link, which can be used to edit bot description; may be null
    pub edit_description_link: Option<crate::enums::InternalLinkType>,
    /// The internal link, which can be used to edit the photo or animation shown in the chat with the bot if the chat is empty; may be null
    pub edit_description_media_link: Option<crate::enums::InternalLinkType>,
    /// The internal link, which can be used to edit bot settings; may be null
    pub edit_settings_link: Option<crate::enums::InternalLinkType>,
}

/// Returns bot members of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembersFilterBots {
}

/// Returns bot members of the supergroup or channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterBots {
}

/// An approval from a guard bot through a Web App is required to join the chat
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinResultGuardBotApprovalRequired {
    /// Identifier of the guard bot
    pub bot_user_id: i64,
    /// Unique identifier of the join request, which will be used in getGuardBotWebAppUrl and updateChatJoinResult
    #[serde_as(as = "DisplayFromStr")]
    pub query_id: i64,
}

/// A button that requests creation of a managed bot by the current user; available only in private chats. Use the method createBot to complete the request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestManagedBot {
    /// Unique button identifier
    pub id: i32,
    /// Suggested name for the bot; may be empty if not specified
    pub suggested_name: String,
    /// Suggested username for the bot; may be empty if not specified
    pub suggested_username: String,
}

/// A bot command
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextBotCommand {
    /// Text
    pub text: crate::enums::RichText,
    /// The bot command
    pub bot_command: String,
}

/// The content must be bottom-aligned
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockVerticalAlignmentBottom {
}

/// The link is a link to a dialog for creating of a managed bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeRequestManagedBot {
}

/// A bot managed by another bot was created by the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageManagedBotCreated {
    /// User identifier of the created bot
    pub bot_user_id: i64,
    /// Identifier of the bot which will manage the new bot
    pub manager_bot_user_id: i64,
}

/// The user allowed the bot to send messages
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageBotWriteAccessAllowed {
    /// The reason why the bot was allowed to write messages
    pub reason: crate::enums::BotWriteAccessAllowReason,
}

/// A bot command, beginning with "/"
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeBotCommand {
}

/// Returns only private chats with bots
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchChatTypeFilterBot {
}

/// Describes media previews of a bot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BotMediaPreview {
    /// Point in time (Unix timestamp) when the preview was added or changed last time
    pub date: i32,
    /// Content of the preview; may only be of the types storyContentPhoto, storyContentVideo, or storyContentUnsupported
    pub content: crate::enums::StoryContent,
}

/// Contains a list of media previews of a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotMediaPreviews {
    /// List of media previews
    pub previews: Vec<crate::types::BotMediaPreview>,
}

/// Contains a list of media previews of a bot for the given language and the list of languages for which the bot has dedicated previews
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotMediaPreviewInfo {
    /// List of media previews
    pub previews: Vec<crate::types::BotMediaPreview>,
    /// List of language codes for which the bot has dedicated previews
    pub language_codes: Vec<String>,
}

/// Describes a color to highlight a bot added to attachment menu
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AttachmentMenuBotColor {
    /// Color in the RGB format for light themes
    pub light_color: i32,
    /// Color in the RGB format for dark themes
    pub dark_color: i32,
}

/// Represents a bot, which can be added to attachment or side menu
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AttachmentMenuBot {
    /// User identifier of the bot
    pub bot_user_id: i64,
    /// True, if the bot supports opening from attachment menu in the chat with the bot
    pub supports_self_chat: bool,
    /// True, if the bot supports opening from attachment menu in private chats with ordinary users
    pub supports_user_chats: bool,
    /// True, if the bot supports opening from attachment menu in private chats with other bots
    pub supports_bot_chats: bool,
    /// True, if the bot supports opening from attachment menu in basic group and supergroup chats
    pub supports_group_chats: bool,
    /// True, if the bot supports opening from attachment menu in channel chats
    pub supports_channel_chats: bool,
    /// True, if the user must be asked for the permission to send messages to the bot
    pub request_write_access: bool,
    /// True, if the bot was explicitly added by the user. If the bot isn't added, then on the first bot launch toggleBotIsAddedToAttachmentMenu must be called and the bot must be added or removed
    pub is_added: bool,
    /// True, if the bot must be shown in the attachment menu
    pub show_in_attachment_menu: bool,
    /// True, if the bot must be shown in the side menu
    pub show_in_side_menu: bool,
    /// True, if a disclaimer, why the bot is shown in the side menu, is needed
    pub show_disclaimer_in_side_menu: bool,
    /// Name for the bot in attachment menu
    pub name: String,
    /// Color to highlight selected name of the bot if appropriate; may be null
    pub name_color: Option<crate::types::AttachmentMenuBotColor>,
    /// Default icon for the bot in SVG format; may be null
    pub default_icon: Option<crate::types::File>,
    /// Icon for the bot in SVG format for the official iOS app; may be null
    pub ios_static_icon: Option<crate::types::File>,
    /// Icon for the bot in TGS format for the official iOS app; may be null
    pub ios_animated_icon: Option<crate::types::File>,
    /// Icon for the bot in PNG format for the official iOS app side menu; may be null
    pub ios_side_menu_icon: Option<crate::types::File>,
    /// Icon for the bot in TGS format for the official Android app; may be null
    pub android_icon: Option<crate::types::File>,
    /// Icon for the bot in SVG format for the official Android app side menu; may be null
    pub android_side_menu_icon: Option<crate::types::File>,
    /// Icon for the bot in TGS format for the official native macOS app; may be null
    pub macos_icon: Option<crate::types::File>,
    /// Icon for the bot in PNG format for the official macOS app side menu; may be null
    pub macos_side_menu_icon: Option<crate::types::File>,
    /// Color to highlight selected icon of the bot if appropriate; may be null
    pub icon_color: Option<crate::types::AttachmentMenuBotColor>,
    /// Default placeholder for opened Web Apps in SVG format; may be null
    pub web_app_placeholder: Option<crate::types::File>,
}

/// The user connected a website by logging in using Telegram Login Widget on it
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotWriteAccessAllowReasonConnectedWebsite {
    /// Domain name of the connected website
    pub domain_name: String,
}

/// The user added the bot to attachment or side menu using toggleBotIsAddedToAttachmentMenu
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotWriteAccessAllowReasonAddedToAttachmentMenu {
}

/// The user launched a Web App using getWebAppLinkUrl
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BotWriteAccessAllowReasonLaunchedWebApp {
    /// Information about the Web App
    pub web_app: crate::types::WebApp,
}

/// The user accepted bot's request to send messages with allowBotToSendMessages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotWriteAccessAllowReasonAcceptedRequest {
}

/// Describes the button that opens a private chat with the bot and sends a start message to the bot with the given parameter
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultsButtonTypeStartBot {
    /// The parameter for the bot start message
    pub parameter: String,
}

/// A rule to allow all bots to do something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleAllowBots {
}

/// A rule to restrict all bots from doing something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleRestrictBots {
}

/// A business bot connected to the current user's account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SessionTypeConnectedBot {
    /// User identifier of the bot. Use deleteBusinessConnectedBot to remove it or confirmBusinessConnectedBot to confirm it if it isn't confirmed yet
    pub bot_user_id: i64,
}

/// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat.
/// Then, call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot.
/// If the bot isn't added to attachment menu, then show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu.
/// If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot.
/// If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeAttachmentMenuBot {
    /// Target chat to be opened
    pub target_chat: crate::enums::TargetChat,
    /// Username of the bot
    pub bot_username: String,
    /// URL to be passed to openWebApp
    pub url: String,
}

/// The link is a link to a Telegram bot, which is expected to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot,
/// ask the current user to select a channel chat to add the bot to as an administrator. Then, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator,
/// check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBotAddToChannel {
    /// Username of the bot
    pub bot_username: String,
    /// Expected administrator rights for the bot
    pub administrator_rights: crate::types::ChatAdministratorRights,
}

/// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot,
/// and then call sendBotStartMessage with the given start parameter after the button is pressed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBotStart {
    /// Username of the bot
    pub bot_username: String,
    /// The parameter to be passed to sendBotStartMessage
    pub start_parameter: String,
    /// True, if sendBotStartMessage must be called automatically without showing the START button
    pub autostart: bool,
}

/// The link is a link to a Telegram bot, which is expected to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups,
/// ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup.
/// If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator,
/// check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user,
/// and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat.
/// Then, if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat; otherwise, just send /start message with bot's username added to the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBotStartInGroup {
    /// Username of the bot
    pub bot_username: String,
    /// The parameter to be passed to sendBotStartMessage
    pub start_parameter: String,
    /// Expected administrator rights for the bot; may be null
    pub administrator_rights: Option<crate::types::ChatAdministratorRights>,
}

/// The link is a link to a dialog for creating of a managed bot. Call searchPublicChat with the given manager bot username.
/// If the chat is found, the chat is a chat with a bot and the bot has can_manage_bots == true, then show bot creation confirmation dialog
/// with the given suggested_bot_username and suggested_bot_name. If user agrees, call createBot with via_link == true to create the bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeRequestManagedBot {
    /// Username of the bot which will manage the new bot
    pub manager_bot_username: String,
    /// Suggested username for the bot; always ends with "bot" case-insensitive
    pub suggested_bot_username: String,
    /// Suggested name for the bot; may be empty if not specified
    pub suggested_bot_name: String,
}

/// A category containing frequently used private chats with bot users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryBots {
}

/// A category containing frequently used chats with inline bots sorted by their usage in inline mode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryInlineBots {
}

/// A category containing frequently used chats with bots, which were used as guest bots
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryGuestBots {
}

/// A category containing frequently used chats with bots, which Web Apps were opened
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryWebAppBots {
}

/// A scope covering all users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeDefault {
}

/// A scope covering all private chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeAllPrivateChats {
}

/// A scope covering all group and supergroup chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeAllGroupChats {
}

/// A scope covering all group and supergroup chat administrators
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeAllChatAdministrators {
}

/// A scope covering all members of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeChat {
    /// Chat identifier
    pub chat_id: i64,
}

/// A scope covering all administrators of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeChatAdministrators {
    /// Chat identifier
    pub chat_id: i64,
}

/// A scope covering a member of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeChatMember {
    /// Chat identifier
    pub chat_id: i64,
    /// User identifier
    pub user_id: i64,
}

/// Lists of bots which Mini Apps must be allowed to read text from clipboard and must be opened without a warning
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateTrustedMiniAppBots {
    /// List of user identifiers of the bots; the corresponding users may not be sent using updateUser updates and may not be accessible
    pub bot_user_ids: Vec<i64>,
}

/// The list of bots added to attachment or side menu has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAttachmentMenuBots {
    /// The new list of bots. The bots must not be shown on scheduled messages screen
    pub bots: Vec<crate::types::AttachmentMenuBot>,
}

/// A bot that can be managed by the current bot was created or updated; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateManagedBot {
    /// Identifier of the user who created the bot
    pub user_id: i64,
    /// Identifier of the created managed bot
    pub bot_user_id: i64,
}


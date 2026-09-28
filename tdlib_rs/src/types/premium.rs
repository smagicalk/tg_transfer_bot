//!
//! TDLib `premium` domain types.
//!
//! Types, enums, and functions for Telegram Premium, chat boosts, giveaways, and business features.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user must buy Telegram Premium as an in-store purchase to log in. Call checkAuthenticationPremiumPurchase and then setAuthenticationPremiumPurchaseTransaction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitPremiumPurchase {
    /// Identifier of the store product that must be bought
    pub store_product_id: String,
    /// Duration of the Telegram Premium subscription after the purchase; may be 0 if Telegram Premium subscription will not be granted
    pub premium_day_count: i32,
    /// Email address to use for support if the user has issues with Telegram Premium purchase
    pub support_email_address: String,
    /// Subject for the email sent to the support email address
    pub support_email_subject: String,
}

/// Send away messages always
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessAwayMessageScheduleAlways {
}

/// Send away messages outside of the business opening hours
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessAwayMessageScheduleOutsideOfOpeningHours {
}

/// Send away messages only in the specified time span
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessAwayMessageScheduleCustom {
    /// Point in time (Unix timestamp) when the away messages will start to be sent
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the away messages will stop to be sent
    pub end_date: i32,
}

/// Represents a location of a business
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessLocation {
    /// The location; may be null if not specified
    pub location: Option<crate::types::Location>,
    /// Location address; 1-96 characters
    pub address: String,
}

/// Describes private chats chosen for automatic interaction with a business
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessRecipients {
    /// Identifiers of selected private chats
    pub chat_ids: Vec<i64>,
    /// Identifiers of private chats that are always excluded; for businessConnectedBot only
    pub excluded_chat_ids: Vec<i64>,
    /// True, if all existing private chats are selected
    pub select_existing_chats: bool,
    /// True, if all new private chats are selected
    pub select_new_chats: bool,
    /// True, if all private chats with contacts are selected
    pub select_contacts: bool,
    /// True, if all private chats with non-contacts are selected
    pub select_non_contacts: bool,
    /// If true, then all private chats except the selected are chosen. Otherwise, only the selected chats are chosen
    pub exclude_selected: bool,
}

/// Describes settings for messages that are automatically sent by a Telegram Business account when it is away
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessAwayMessageSettings {
    /// Unique quick reply shortcut identifier for the away messages
    pub shortcut_id: i32,
    /// Chosen recipients of the away messages
    pub recipients: crate::types::BusinessRecipients,
    /// Settings used to check whether the current user is away
    pub schedule: crate::enums::BusinessAwayMessageSchedule,
    /// True, if the messages must not be sent if the account was online in the last 10 minutes
    pub offline_only: bool,
}

/// Describes settings for greeting messages that are automatically sent by a Telegram Business account as response to incoming messages in an inactive private chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessGreetingMessageSettings {
    /// Unique quick reply shortcut identifier for the greeting messages
    pub shortcut_id: i32,
    /// Chosen recipients of the greeting messages
    pub recipients: crate::types::BusinessRecipients,
    /// The number of days after which a chat will be considered as inactive; currently, must be one of 7, 14, 21, or 28
    pub inactivity_days: i32,
}

/// Describes rights of a business bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessBotRights {
    /// True, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    pub can_reply: bool,
    /// True, if the bot can mark incoming private messages as read
    pub can_read_messages: bool,
    /// True, if the bot can delete sent messages
    pub can_delete_sent_messages: bool,
    /// True, if the bot can delete any message
    pub can_delete_all_messages: bool,
    /// True, if the bot can edit name of the business account
    pub can_edit_name: bool,
    /// True, if the bot can edit bio of the business account
    pub can_edit_bio: bool,
    /// True, if the bot can edit profile photo of the business account
    pub can_edit_profile_photo: bool,
    /// True, if the bot can edit username of the business account
    pub can_edit_username: bool,
    /// True, if the bot can view gifts and Telegram Star amount owned by the business account
    pub can_view_gifts_and_stars: bool,
    /// True, if the bot can sell regular gifts received by the business account
    pub can_sell_gifts: bool,
    /// True, if the bot can change gift receiving settings of the business account
    pub can_change_gift_settings: bool,
    /// True, if the bot can transfer and upgrade gifts received by the business account
    pub can_transfer_and_upgrade_gifts: bool,
    /// True, if the bot can transfer Telegram Stars received by the business account to account of the bot, or use them to upgrade and transfer gifts
    pub can_transfer_stars: bool,
    /// True, if the bot can post, edit and delete stories
    pub can_manage_stories: bool,
}

/// Describes a business bot connected to an account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessConnectedBot {
    /// User identifier of the bot
    pub bot_user_id: i64,
    /// Private chats that will be accessible to the bot
    pub recipients: crate::types::BusinessRecipients,
    /// Rights of the bot
    pub rights: crate::types::BusinessBotRights,
}

/// Describes a connection of a bot to an account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessConnectedBotInfo {
    /// Information about the bot
    pub bot: crate::types::BusinessConnectedBot,
    /// Point in time (Unix timestamp) when the bot was added; may be 0 if unknown
    pub connection_date: i32,
    /// Model of the device that was used for the bot connection, as provided by the application; may be empty if unknown
    pub device_model: String,
    /// A human-readable description of the location from which the bot was connected, based on the IP address; may be empty if unknown
    pub location: String,
}

/// Describes settings for a business account start page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessStartPage {
    /// Title text of the start page
    pub title: String,
    /// Message text of the start page
    pub message: String,
    /// Greeting sticker of the start page; may be null if none
    pub sticker: Option<crate::types::Sticker>,
}

/// Describes settings for a business account start page to set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputBusinessStartPage {
    /// Title text of the start page; 0-getOption("business_start_page_title_length_max") characters
    pub title: String,
    /// Message text of the start page; 0-getOption("business_start_page_message_length_max") characters
    pub message: String,
    /// Greeting sticker of the start page; pass null if none. The sticker must belong to a sticker set and must not be a custom emoji
    pub sticker: Option<crate::enums::InputFile>,
}

/// Describes an interval of time when the business is open
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessOpeningHoursInterval {
    /// The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0-7*24*60
    pub start_minute: i32,
    /// The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 1-8*24*60
    pub end_minute: i32,
}

/// Describes opening hours of a business
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessOpeningHours {
    /// Unique time zone identifier
    pub time_zone_id: String,
    /// Intervals of the time when the business is open
    pub opening_hours: Vec<crate::types::BusinessOpeningHoursInterval>,
}

/// Contains information about a Telegram Business account
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessInfo {
    /// Location of the business; may be null if none
    pub location: Option<crate::types::BusinessLocation>,
    /// Opening hours of the business; may be null if none. The hours are guaranteed to be valid and have already been split by week days
    pub opening_hours: Option<crate::types::BusinessOpeningHours>,
    /// Opening hours of the business in the local time; may be null if none. The hours are guaranteed to be valid and have already been split by week days.
    /// Local time zone identifier will be empty. An updateUserFullInfo update is not triggered when value of this field changes
    pub local_opening_hours: Option<crate::types::BusinessOpeningHours>,
    /// Time left before the business will open the next time, in seconds; 0 if unknown. An updateUserFullInfo update is not triggered when value of this field changes
    pub next_open_in: i32,
    /// Time left before the business will close the next time, in seconds; 0 if unknown. An updateUserFullInfo update is not triggered when value of this field changes
    pub next_close_in: i32,
    /// The greeting message; may be null if none or the Business account is not of the current user
    pub greeting_message_settings: Option<crate::types::BusinessGreetingMessageSettings>,
    /// The away message; may be null if none or the Business account is not of the current user
    pub away_message_settings: Option<crate::types::BusinessAwayMessageSettings>,
    /// Information about start page of the account; may be null if none
    pub start_page: Option<crate::types::BusinessStartPage>,
}

/// Contains information about a business chat link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessChatLink {
    /// The HTTPS link
    pub link: String,
    /// Message draft text that will be added to the input field
    pub text: crate::types::FormattedText,
    /// Link title
    pub title: String,
    /// Number of times the link was used
    pub view_count: i32,
}

/// Contains a list of business chat links created by the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessChatLinks {
    /// List of links
    pub links: Vec<crate::types::BusinessChatLink>,
}

/// Describes a business chat link to create or edit
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputBusinessChatLink {
    /// Message draft text that will be added to the input field
    pub text: crate::types::FormattedText,
    /// Link title
    pub title: String,
}

/// Contains information about a business chat link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessChatLinkInfo {
    /// Identifier of the private chat that created the link
    pub chat_id: i64,
    /// Message draft text that must be added to the input field
    pub text: crate::types::FormattedText,
}

/// The affiliate is the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateTypeCurrentUser {
}

/// The affiliate is a bot owned by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateTypeBot {
    /// User identifier of the bot
    pub user_id: i64,
}

/// The affiliate is a channel chat where the current user has can_post_messages administrator right
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateTypeChannel {
    /// Identifier of the channel chat
    pub chat_id: i64,
}

/// The affiliate programs must be sorted by the profitability
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateProgramSortOrderProfitability {
}

/// The affiliate programs must be sorted by creation date
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateProgramSortOrderCreationDate {
}

/// Describes parameters of an affiliate program
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateProgramParameters {
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner;
    /// getOption("affiliate_program_commission_per_mille_min")-getOption("affiliate_program_commission_per_mille_max")
    pub commission_per_mille: i32,
    /// Number of months the program will be active; 0-36. If 0, then the program is eternal
    pub month_count: i32,
}

/// Contains information about an active affiliate program
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateProgramInfo {
    /// Parameters of the affiliate program
    pub parameters: crate::types::AffiliateProgramParameters,
    /// Point in time (Unix timestamp) when the affiliate program will be closed; 0 if the affiliate program isn't scheduled to be closed.
    /// If positive, then the program can't be connected using connectAffiliateProgram, but active connections will work until the date
    pub end_date: i32,
    /// The amount of daily revenue per user in Telegram Stars of the bot that created the affiliate program
    pub daily_revenue_per_user_amount: crate::types::StarAmount,
}

/// Contains information about an affiliate that received commission from a Telegram Star transaction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateInfo {
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner
    pub commission_per_mille: i32,
    /// Identifier of the chat which received the commission
    pub affiliate_chat_id: i64,
    /// The Telegram Star amount that was received by the affiliate; can be negative for refunds
    pub star_amount: crate::types::StarAmount,
}

/// Describes a found affiliate program
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundAffiliateProgram {
    /// User identifier of the bot created the program
    pub bot_user_id: i64,
    /// Information about the affiliate program
    pub info: crate::types::AffiliateProgramInfo,
}

/// Represents a list of found affiliate programs
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundAffiliatePrograms {
    /// The total number of found affiliate programs
    pub total_count: i32,
    /// The list of affiliate programs
    pub programs: Vec<crate::types::FoundAffiliateProgram>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Describes an affiliate program that was connected to an affiliate
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedAffiliateProgram {
    /// The link that can be used to refer users if the program is still active
    pub url: String,
    /// User identifier of the bot created the program
    pub bot_user_id: i64,
    /// The parameters of the affiliate program
    pub parameters: crate::types::AffiliateProgramParameters,
    /// Point in time (Unix timestamp) when the affiliate program was connected
    pub connection_date: i32,
    /// True, if the program was canceled by the bot, or disconnected by the chat owner and isn't available anymore
    pub is_disconnected: bool,
    /// The number of users that used the affiliate program
    #[serde_as(as = "DisplayFromStr")]
    pub user_count: i64,
    /// The number of Telegram Stars that were earned by the affiliate program
    pub revenue_star_count: i64,
}

/// Represents a list of affiliate programs that were connected to an affiliate
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedAffiliatePrograms {
    /// The total number of affiliate programs that were connected to the affiliate
    pub total_count: i32,
    /// The list of connected affiliate programs
    pub programs: Vec<crate::types::ConnectedAffiliateProgram>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Contains information about a Telegram Premium gift code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumGiftCodeInfo {
    /// Identifier of a chat or a user who created the gift code; may be null if unknown. If null and the code is from messagePremiumGiftCode message, then creator_id from the message can be used
    pub creator_id: Option<crate::enums::MessageSender>,
    /// Point in time (Unix timestamp) when the code was created
    pub creation_date: i32,
    /// True, if the gift code was created for a giveaway
    pub is_from_giveaway: bool,
    /// Identifier of the corresponding giveaway message in the creator_id chat; may be 0 or an identifier of a deleted message
    pub giveaway_message_id: i64,
    /// Number of months the Telegram Premium subscription will be active after code activation; 0 if the number of months isn't integer
    pub month_count: i32,
    /// Number of days the Telegram Premium subscription will be active after code activation
    pub day_count: i32,
    /// Identifier of a user for which the code was created; 0 if none
    pub user_id: i64,
    /// Point in time (Unix timestamp) when the code was activated; 0 if none
    pub use_date: i32,
}

/// Describes an option for the number of winners of a Telegram Star giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarGiveawayWinnerOption {
    /// The number of users that will be chosen as winners
    pub winner_count: i32,
    /// The number of Telegram Stars that will be won by the winners of the giveaway
    pub won_star_count: i64,
    /// True, if the option must be chosen by default
    pub is_default: bool,
}

/// The transaction is a deposit of Telegram Stars from the Premium bot; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePremiumBotDeposit {
}

/// The transaction is a deposit of Telegram Stars from a giveaway; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiveawayDeposit {
    /// Identifier of a supergroup or a channel chat that created the giveaway
    pub chat_id: i64,
    /// Identifier of the message with the giveaway; may be 0 or an identifier of a deleted message
    pub giveaway_message_id: i64,
}

/// The transaction is a receiving of a commission from an affiliate program; relevant for regular users, bots and channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeAffiliateProgramCommission {
    /// Identifier of the chat that created the affiliate program
    pub chat_id: i64,
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner
    pub commission_per_mille: i32,
}

/// The transaction is a purchase of Telegram Premium subscription; relevant for regular users and bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePremiumPurchase {
    /// Identifier of the user who received the Telegram Premium subscription
    pub user_id: i64,
    /// Number of months the Telegram Premium subscription will be active
    pub month_count: i32,
    /// A sticker to be shown in the transaction information; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// The transaction is a transfer of Telegram Stars to a business bot; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBusinessBotTransferSend {
    /// Identifier of the bot that received Telegram Stars
    pub user_id: i64,
}

/// The transaction is a transfer of Telegram Stars from a business account; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBusinessBotTransferReceive {
    /// Identifier of the user who sent Telegram Stars
    pub user_id: i64,
}

/// The user is eligible for the giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParticipantStatusEligible {
}

/// The user participates in the giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParticipantStatusParticipating {
}

/// The user can't participate in the giveaway, because they have already been member of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParticipantStatusAlreadyWasMember {
    /// Point in time (Unix timestamp) when the user joined the chat
    pub joined_chat_date: i32,
}

/// The user can't participate in the giveaway, because they are an administrator in one of the chats that created the giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParticipantStatusAdministrator {
    /// Identifier of the chat administered by the user
    pub chat_id: i64,
}

/// The user can't participate in the giveaway, because their phone number is from a disallowed country
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParticipantStatusDisallowedCountry {
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's country
    pub user_country_code: String,
}

/// Describes an ongoing giveaway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiveawayInfoOngoing {
    /// Point in time (Unix timestamp) when the giveaway was created
    pub creation_date: i32,
    /// Status of the current user in the giveaway
    pub status: crate::enums::GiveawayParticipantStatus,
    /// True, if the giveaway has ended and results are being prepared
    pub is_ended: bool,
}

/// Describes a completed giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayInfoCompleted {
    /// Point in time (Unix timestamp) when the giveaway was created
    pub creation_date: i32,
    /// Point in time (Unix timestamp) when the winners were selected. May be bigger than winners selection date specified in parameters of the giveaway
    pub actual_winners_selection_date: i32,
    /// True, if the giveaway was canceled and was fully refunded
    pub was_refunded: bool,
    /// True, if the current user is a winner of the giveaway
    pub is_winner: bool,
    /// Number of winners in the giveaway
    pub winner_count: i32,
    /// Number of winners, which activated their gift codes; for Telegram Premium giveaways only
    pub activation_count: i32,
    /// Telegram Premium gift code that was received by the current user; empty if the user isn't a winner in the giveaway or the giveaway isn't a Telegram Premium giveaway
    pub gift_code: String,
    /// The Telegram Star amount won by the current user; 0 if the user isn't a winner in the giveaway or the giveaway isn't a Telegram Star giveaway
    pub won_star_count: i64,
}

/// The giveaway sends Telegram Premium subscriptions to the winners
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayPrizePremium {
    /// Number of months the Telegram Premium subscription will be active after code activation
    pub month_count: i32,
}

/// Describes a message from a business account as received by a bot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessMessage {
    /// The message
    pub message: crate::types::Message,
    /// Message that is replied by the message in the same chat; may be null if none
    pub reply_to_message: Option<crate::types::Message>,
}

/// Contains a list of messages from a business account as received by a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessMessages {
    /// List of business messages
    pub messages: Vec<crate::types::BusinessMessage>,
}

/// The user asked to hide sponsored messages, but Telegram Premium is required for this
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportSponsoredResultPremiumRequired {
}

/// Contains information about a business bot that manages the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessBotManageBar {
    /// User identifier of the bot
    pub bot_user_id: i64,
    /// URL to be opened to manage the bot
    pub manage_url: String,
    /// True, if the bot is paused. Use toggleBusinessConnectedBotChatIsPaused to change the value of the field
    pub is_bot_paused: bool,
    /// True, if the bot can reply
    pub can_bot_reply: bool,
}

/// The link is a link to boost a channel chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeChannelBoost {
    /// Photo of the chat; may be null
    pub photo: Option<crate::types::ChatPhoto>,
}

/// The link is a link to a Telegram Premium gift code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypePremiumGiftCode {
}

/// The link is a link to boost a supergroup chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeSupergroupBoost {
    /// Photo of the chat; may be null
    pub photo: Option<crate::types::ChatPhoto>,
}

/// Describes parameters of a giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParameters {
    /// Identifier of the supergroup or channel chat, which will be automatically boosted by the winners of the giveaway for duration of the Telegram Premium subscription,
    /// or for the specified time. If the chat is a channel, then can_post_messages administrator right is required in the channel, otherwise, the user must be an administrator in the supergroup
    pub boosted_chat_id: i64,
    /// Identifiers of other supergroup or channel chats that must be subscribed by the users to be eligible for the giveaway. There can be up to getOption("giveaway_additional_chat_count_max") additional chats
    pub additional_chat_ids: Vec<i64>,
    /// Point in time (Unix timestamp) when the giveaway is expected to be performed; must be 60-getOption("giveaway_duration_max") seconds in the future in scheduled giveaways
    pub winners_selection_date: i32,
    /// True, if only new members of the chats will be eligible for the giveaway
    pub only_new_members: bool,
    /// True, if the list of winners of the giveaway will be available to everyone
    pub has_public_winners: bool,
    /// The list of two-letter ISO 3166-1 alpha-2 codes of countries, users from which will be eligible for the giveaway. If empty, then all users can participate in the giveaway.
    /// There can be up to getOption("giveaway_country_count_max") chosen countries. Users with phone number that was bought at https:fragment.com can participate in any giveaway and the country code "FT" must not be specified in the list
    pub country_codes: Vec<String>,
    /// Additional description of the giveaway prize; 0-128 characters
    pub prize_description: String,
}

/// The chat was boosted by the sender of the message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatBoost {
    /// Number of times the chat was boosted
    pub boost_count: i32,
}

/// Telegram Premium was gifted to a user
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiftedPremium {
    /// The identifier of a user who gifted Telegram Premium; 0 if the gift was anonymous or is outgoing
    pub gifter_user_id: i64,
    /// The identifier of a user who received Telegram Premium; 0 if the gift is incoming
    pub receiver_user_id: i64,
    /// Message added to the gifted Telegram Premium by the sender
    pub text: crate::types::FormattedText,
    /// Currency for the paid amount
    pub currency: String,
    /// The paid amount, in the smallest units of the currency
    pub amount: i64,
    /// Cryptocurrency used to pay for the gift; may be empty if none
    pub cryptocurrency: String,
    /// The paid amount, in the smallest units of the cryptocurrency; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Number of months the Telegram Premium subscription will be active after code activation; 0 if the number of months isn't integer
    pub month_count: i32,
    /// Number of days the Telegram Premium subscription will be active
    pub day_count: i32,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// A Telegram Premium gift code was created for the user
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessagePremiumGiftCode {
    /// Identifier of a chat or a user who created the gift code; may be null if unknown
    pub creator_id: Option<crate::enums::MessageSender>,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// True, if the gift code was created for a giveaway
    pub is_from_giveaway: bool,
    /// True, if the winner for the corresponding Telegram Premium subscription wasn't chosen
    pub is_unclaimed: bool,
    /// Currency for the paid amount; empty if unknown
    pub currency: String,
    /// The paid amount, in the smallest units of the currency; 0 if unknown
    pub amount: i64,
    /// Cryptocurrency used to pay for the gift; may be empty if none or unknown
    pub cryptocurrency: String,
    /// The paid amount, in the smallest units of the cryptocurrency; 0 if unknown
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Number of months the Telegram Premium subscription will be active after code activation; 0 if the number of months isn't integer
    pub month_count: i32,
    /// Number of days the Telegram Premium subscription will be active after code activation
    pub day_count: i32,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
    /// The gift code
    pub code: String,
}

/// A giveaway was created for the chat. Use telegramPaymentPurposePremiumGiveaway, storePaymentPurposePremiumGiveaway, telegramPaymentPurposeStarGiveaway, or storePaymentPurposeStarGiveaway to create a giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveawayCreated {
    /// Number of Telegram Stars that will be shared by winners of the giveaway; 0 for Telegram Premium giveaways
    pub star_count: i64,
}

/// A giveaway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveaway {
    /// Giveaway parameters
    pub parameters: crate::types::GiveawayParameters,
    /// Number of users who will receive Telegram Premium subscription gift codes
    pub winner_count: i32,
    /// Prize of the giveaway
    pub prize: crate::enums::GiveawayPrize,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// A giveaway without public winners has been completed for the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveawayCompleted {
    /// Identifier of the message with the giveaway; may be 0 or an identifier of a deleted message
    pub giveaway_message_id: i64,
    /// Number of winners in the giveaway
    pub winner_count: i32,
    /// True, if the giveaway is a Telegram Star giveaway
    pub is_star_giveaway: bool,
    /// Number of undistributed prizes; for Telegram Premium giveaways only
    pub unclaimed_prize_count: i32,
}

/// A giveaway with public winners has been completed for the chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiveawayWinners {
    /// Identifier of the supergroup or channel chat, which was automatically boosted by the winners of the giveaway
    pub boosted_chat_id: i64,
    /// Identifier of the message with the giveaway in the boosted chat
    pub giveaway_message_id: i64,
    /// Number of other chats that participated in the giveaway
    pub additional_chat_count: i32,
    /// Point in time (Unix timestamp) when the winners were selected. May be bigger than winners selection date specified in parameters of the giveaway
    pub actual_winners_selection_date: i32,
    /// True, if only new members of the chats were eligible for the giveaway
    pub only_new_members: bool,
    /// True, if the giveaway was canceled and was fully refunded
    pub was_refunded: bool,
    /// Prize of the giveaway
    pub prize: crate::enums::GiveawayPrize,
    /// Additional description of the giveaway prize
    pub prize_description: String,
    /// Total number of winners in the giveaway
    pub winner_count: i32,
    /// Up to 100 user identifiers of the winners of the giveaway
    pub winner_user_ids: Vec<i64>,
    /// Number of undistributed prizes; for Telegram Premium giveaways only
    pub unclaimed_prize_count: i32,
}

/// Contains a list of features available on a specific chat boost level
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostLevelFeatures {
    /// Target chat boost level
    pub level: i32,
    /// Number of stories that the chat can publish daily
    pub story_per_day_count: i32,
    /// Number of custom emoji reactions that can be added to the list of available reactions
    pub custom_emoji_reaction_count: i32,
    /// Number of custom colors for chat title
    pub title_color_count: i32,
    /// Number of custom colors for profile photo background
    pub profile_accent_color_count: i32,
    /// True, if custom emoji for profile background can be set
    pub can_set_profile_background_custom_emoji: bool,
    /// Number of custom colors for background of empty chat photo, replies to messages and link previews
    pub accent_color_count: i32,
    /// True, if custom emoji for reply header and link preview background can be set
    pub can_set_background_custom_emoji: bool,
    /// True, if emoji status can be set
    pub can_set_emoji_status: bool,
    /// Number of chat theme backgrounds that can be set as chat background
    pub chat_theme_background_count: i32,
    /// True, if custom background can be set in the chat for all users
    pub can_set_custom_background: bool,
    /// True, if custom emoji sticker set can be set for the chat
    pub can_set_custom_emoji_sticker_set: bool,
    /// True, if automatic translation of messages can be enabled in the chat
    pub can_enable_automatic_translation: bool,
    /// True, if speech recognition can be used for video note and voice note messages by all users
    pub can_recognize_speech: bool,
    /// True, if sponsored messages can be disabled in the chat
    pub can_disable_sponsored_messages: bool,
}

/// Contains a list of features available on the first chat boost levels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostFeatures {
    /// The list of features
    pub features: Vec<crate::types::ChatBoostLevelFeatures>,
    /// The minimum boost level required to set custom emoji for profile background
    pub min_profile_background_custom_emoji_boost_level: i32,
    /// The minimum boost level required to set custom emoji for reply header and link preview background; for channel chats only
    pub min_background_custom_emoji_boost_level: i32,
    /// The minimum boost level required to set emoji status
    pub min_emoji_status_boost_level: i32,
    /// The minimum boost level required to set a chat theme background as chat background
    pub min_chat_theme_background_boost_level: i32,
    /// The minimum boost level required to set custom chat background
    pub min_custom_background_boost_level: i32,
    /// The minimum boost level required to set custom emoji sticker set for the chat; for supergroup chats only
    pub min_custom_emoji_sticker_set_boost_level: i32,
    /// The minimum boost level allowing to enable automatic translation of messages for non-Premium users; for channel chats only
    pub min_automatic_translation_boost_level: i32,
    /// The minimum boost level allowing to recognize speech in video note and voice note messages for non-Premium users; for supergroup chats only
    pub min_speech_recognition_boost_level: i32,
    /// The minimum boost level allowing to disable sponsored messages in the chat; for channel chats only
    pub min_sponsored_message_disable_boost_level: i32,
}

/// The chat created a Telegram Premium gift code for a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSourceGiftCode {
    /// Identifier of a user, for which the gift code was created
    pub user_id: i64,
    /// The created Telegram Premium gift code, which is known only if this is a gift code for the current user, or it has already been claimed
    pub gift_code: String,
}

/// The chat created a giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a user who won in the giveaway; 0 if none
    pub user_id: i64,
    /// The created Telegram Premium gift code if it was used by the user or can be claimed by the current user; an empty string otherwise; for Telegram Premium giveways only
    pub gift_code: String,
    /// Number of Telegram Stars distributed among winners of the giveaway
    pub star_count: i64,
    /// Identifier of the corresponding giveaway message; can be an identifier of a deleted message
    pub giveaway_message_id: i64,
    /// True, if the winner for the corresponding giveaway prize wasn't chosen, because there were not enough participants
    pub is_unclaimed: bool,
}

/// A user with Telegram Premium subscription or gifted Telegram Premium boosted the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSourcePremium {
    /// Identifier of the user
    pub user_id: i64,
}

/// Describes a prepaid giveaway
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PrepaidGiveaway {
    /// Unique identifier of the prepaid giveaway
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Number of users who will receive giveaway prize
    pub winner_count: i32,
    /// Prize of the giveaway
    pub prize: crate::enums::GiveawayPrize,
    /// The number of boosts received by the chat from the giveaway; for Telegram Star giveaways only
    pub boost_count: i32,
    /// Point in time (Unix timestamp) when the giveaway was paid
    pub payment_date: i32,
}

/// Describes current boost status of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostStatus {
    /// An HTTP URL, which can be used to boost the chat
    pub boost_url: String,
    /// Identifiers of boost slots of the current user applied to the chat
    pub applied_slot_ids: Vec<i32>,
    /// Current boost level of the chat
    pub level: i32,
    /// The number of boosts received by the chat from created Telegram Premium gift codes and giveaways; always 0 if the current user isn't an administrator in the chat
    pub gift_code_boost_count: i32,
    /// The number of boosts received by the chat
    pub boost_count: i32,
    /// The number of boosts added to reach the current level
    pub current_level_boost_count: i32,
    /// The number of boosts needed to reach the next level; 0 if the next level isn't available
    pub next_level_boost_count: i32,
    /// Approximate number of Telegram Premium subscribers joined the chat; always 0 if the current user isn't an administrator in the chat
    pub premium_member_count: i32,
    /// A percentage of Telegram Premium subscribers joined the chat; always 0 if the current user isn't an administrator in the chat
    pub premium_member_percentage: f64,
    /// The list of prepaid giveaways available for the chat; only for chat administrators
    pub prepaid_giveaways: Vec<crate::types::PrepaidGiveaway>,
}

/// Describes a boost applied to a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatBoost {
    /// Unique identifier of the boost
    pub id: String,
    /// The number of identical boosts applied
    pub count: i32,
    /// Source of the boost
    pub source: crate::enums::ChatBoostSource,
    /// Point in time (Unix timestamp) when the chat was boosted
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the boost will expire
    pub expiration_date: i32,
}

/// Contains a list of boosts applied to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundChatBoosts {
    /// Total number of boosts applied to the chat
    pub total_count: i32,
    /// List of boosts
    pub boosts: Vec<crate::types::ChatBoost>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Describes a slot for chat boost
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSlot {
    /// Unique identifier of the slot
    pub slot_id: i32,
    /// Identifier of the currently boosted chat; 0 if none
    pub currently_boosted_chat_id: i64,
    /// Point in time (Unix timestamp) when the chat was boosted; 0 if none
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the boost will expire
    pub expiration_date: i32,
    /// Point in time (Unix timestamp) after which the boost can be used for another chat
    pub cooldown_until_date: i32,
}

/// Contains a list of chat boost slots
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSlots {
    /// List of boost slots
    pub slots: Vec<crate::types::ChatBoostSlot>,
}

/// Describes a connection of the bot with a business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessConnection {
    /// Unique identifier of the connection
    pub id: String,
    /// Identifier of the business user who created the connection
    pub user_id: i64,
    /// Chat identifier of the private chat with the user
    pub user_chat_id: i64,
    /// Point in time (Unix timestamp) when the connection was established
    pub date: i32,
    /// Rights of the bot; may be null if the connection was disabled
    pub rights: Option<crate::types::BusinessBotRights>,
    /// True, if the connection is enabled; false otherwise
    pub is_enabled: bool,
}

/// The maximum number of joined supergroups and channels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeSupergroupCount {
}

/// The maximum number of pinned chats in the main chat list
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypePinnedChatCount {
}

/// The maximum number of created public chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeCreatedPublicChatCount {
}

/// The maximum number of saved animations
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeSavedAnimationCount {
}

/// The maximum number of chat folders
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeChatFolderCount {
}

/// The maximum number of pinned and always included, or always excluded chats in a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeChatFolderChosenChatCount {
}

/// The maximum number of pinned chats in the archive chat list
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypePinnedArchivedChatCount {
}

/// The maximum number of pinned Saved Messages topics
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypePinnedSavedMessagesTopicCount {
}

/// The maximum length of text of sent messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeMessageTextLength {
}

/// The maximum length of sent media caption
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeCaptionLength {
}

/// The maximum length of the user's bio
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeBioLength {
}

/// The maximum number of invite links for a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeChatFolderInviteLinkCount {
}

/// The maximum number of added shareable chat folders
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeShareableChatFolderCount {
}

/// The maximum number of received similar chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeSimilarChatCount {
}

/// The maximum number of owned bots
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeOwnedBotCount {
}

/// The maximum number of added text composition styles
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeCustomTextCompositionStyleCount {
}

/// Increased limits
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureIncreasedLimits {
}

/// Increased maximum upload file size
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureIncreasedUploadFileSize {
}

/// Improved download speed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureImprovedDownloadSpeed {
}

/// The ability to convert voice notes to text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureVoiceRecognition {
}

/// Disabled ads
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureDisabledAds {
}

/// Allowed to use more reactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureUniqueReactions {
}

/// Ability to change position of the main chat list, archive and mute all new chats from non-contacts, and completely disable notifications about the user's contacts joined Telegram
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureAdvancedChatManagement {
}

/// A badge in the user's profile
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureProfileBadge {
}

/// The ability to set a custom emoji as a forum topic icon
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureForumTopicIcon {
}

/// Allowed to set a premium application icons
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureAppIcons {
}

/// Allowed to translate chat messages real-time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureRealTimeChatTranslation {
}

/// The ability to boost chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureChatBoost {
}

/// The ability to choose accent color for replies and user profile
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureAccentColor {
}

/// The ability to set private chat background for both users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureBackgroundForBoth {
}

/// The ability to use tags in Saved Messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureSavedMessagesTags {
}

/// The ability to disallow incoming voice and video note messages in private chats using setUserPrivacySettingRules with userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages
/// and to restrict incoming messages from non-contacts using setNewChatPrivacySettings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureMessagePrivacy {
}

/// The ability to view last seen and read times of other users even if they can't view last seen or read time for the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureLastSeenTimes {
}

/// The ability to use Business features
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureBusiness {
}

/// The ability to use all available message effects
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureMessageEffects {
}

/// The ability to create and use checklist messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureChecklists {
}

/// The ability to require a payment for incoming messages in new chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeaturePaidMessages {
}

/// The ability to enable content protection in private chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureProtectPrivateChatContent {
}

/// The ability to compose text with AI
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureTextComposition {
}

/// The ability to send rich messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureRichMessages {
}

/// The ability to set location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureLocation {
}

/// The ability to set opening hours
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureOpeningHours {
}

/// The ability to use quick replies
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureQuickReplies {
}

/// The ability to set up a greeting message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureGreetingMessage {
}

/// The ability to set up an away message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureAwayMessage {
}

/// The ability to create links to the business account with predefined message text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureAccountLinks {
}

/// The ability to customize start page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureStartPage {
}

/// The ability to connect a bot to the account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureBots {
}

/// The ability to display folder names for each chat in the chat list
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureChatFolderTags {
}

/// Contains information about a limit, increased for Premium users
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimit {
    /// The type of the limit
    #[serde(rename = "type")]
    pub r#type: crate::enums::PremiumLimitType,
    /// Default value of the limit
    pub default_value: i32,
    /// Value of the limit for Premium users
    pub premium_value: i32,
}

/// Contains information about features, available to Premium users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatures {
    /// The list of available features
    pub features: Vec<crate::enums::PremiumFeature>,
    /// The list of limits, increased for Premium users
    pub limits: Vec<crate::types::PremiumLimit>,
    /// An internal link to be opened to pay for Telegram Premium if store payment isn't possible; may be null if direct payment isn't available
    pub payment_link: Option<crate::enums::InternalLinkType>,
}

/// Contains information about features, available to Business user accounts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatures {
    /// The list of available business features
    pub features: Vec<crate::enums::BusinessFeature>,
}

/// A limit was exceeded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceLimitExceeded {
    /// Type of the exceeded limit
    pub limit_type: crate::enums::PremiumLimitType,
}

/// A user tried to use a Premium feature
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceFeature {
    /// The used feature
    pub feature: crate::enums::PremiumFeature,
}

/// A user tried to use a Business feature
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceBusinessFeature {
    /// The used feature; pass null if none specific feature was used
    pub feature: Option<crate::enums::BusinessFeature>,
}

/// A user opened an internal link of the type internalLinkTypePremiumFeaturesPage
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceLink {
    /// The referrer from the link
    pub referrer: String,
}

/// A user opened the Premium features screen from settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceSettings {
}

/// Describes a promotion animation for a Premium feature
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeaturePromotionAnimation {
    /// Premium feature
    pub feature: crate::enums::PremiumFeature,
    /// Promotion animation for the feature
    pub animation: crate::types::Animation,
}

/// Describes a promotion animation for a Business feature
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeaturePromotionAnimation {
    /// Business feature
    pub feature: crate::enums::BusinessFeature,
    /// Promotion animation for the feature
    pub animation: crate::types::Animation,
}

/// Contains state of Telegram Premium subscription and promotion videos for Premium features
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumState {
    /// Text description of the state of the current Premium subscription; may be empty if the current user has no Telegram Premium subscription
    pub state: crate::types::FormattedText,
    /// The list of available options for buying Telegram Premium
    pub payment_options: Vec<crate::types::PremiumStatePaymentOption>,
    /// The list of available promotion animations for Premium features
    pub animations: Vec<crate::types::PremiumFeaturePromotionAnimation>,
    /// The list of available promotion animations for Business features
    pub business_animations: Vec<crate::types::BusinessFeaturePromotionAnimation>,
}

/// A message with a Telegram Premium gift code created for the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentPremiumGiftCode {
    /// Number of months the Telegram Premium subscription will be active after code activation
    pub month_count: i32,
}

/// A message with a giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentGiveaway {
    /// Number of users who will receive giveaway prizes; 0 for pinned message
    pub winner_count: i32,
    /// Prize of the giveaway; may be null for pinned message
    pub prize: Option<crate::enums::GiveawayPrize>,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A rule to allow all Premium Users to do something; currently, allowed only for userPrivacySettingAllowChatInvites
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleAllowPremiumUsers {
}

/// The "Telegram Business" section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionBusiness {
    /// Subsection of the section; may be one of
    /// "", "do-not-hide-ads"
    pub subsection: String,
}

/// The "Telegram Premium" section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionPremium {
}

/// The link is a link to a business chat. Use getBusinessChatLinkInfo with the provided link name to get information about the link,
/// then open received private chat and replace chat draft with the provided text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBusinessChat {
    /// Name of the link
    pub link_name: String,
}

/// The link is an affiliate program link. Call searchChatAffiliateProgram with the given username and referrer to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatAffiliateProgram {
    /// Username to be passed to searchChatAffiliateProgram
    pub username: String,
    /// Referrer to be passed to searchChatAffiliateProgram
    pub referrer: String,
}

/// The link is a link to boost a Telegram chat. Call getChatBoostLinkInfo with the given URL to process the link.
/// If the chat is found, then call getChatBoostStatus and getAvailableChatBoostSlots to get the current boost status and check whether the chat can be boosted.
/// If the user wants to boost the chat and the chat can be boosted, then call boostChat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatBoost {
    /// URL to be passed to getChatBoostLinkInfo
    pub url: String,
}

/// The link is a link to the Premium features screen of the application from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePremiumFeaturesPage {
    /// Referrer specified in the link
    pub referrer: String,
}

/// The link is a link with a Telegram Premium gift code. Call checkPremiumGiftCode with the given code to process the link.
/// If the code is valid and the user wants to apply it, then call applyPremiumGiftCode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePremiumGiftCode {
    /// The Telegram Premium gift code
    pub code: String,
}

/// The link is a link to the screen for gifting Telegram Premium subscriptions to friends via inputInvoiceTelegram with telegramPaymentPurposePremiumGift payments or in-store purchases
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePremiumGiftPurchase {
    /// Referrer specified in the link
    pub referrer: String,
}

/// Contains an HTTPS link to boost a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostLink {
    /// The link
    pub link: String,
    /// True, if the link will work for non-members of the chat
    pub is_public: bool,
}

/// Contains information about a link to boost a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostLinkInfo {
    /// True, if the link will work for non-members of the chat
    pub is_public: bool,
    /// Identifier of the chat to which the link points; 0 if the chat isn't found
    pub chat_id: i64,
}

/// Suggests the user to upgrade the Premium subscription from monthly payments to annual payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionUpgradePremium {
}

/// Suggests the user to restore a recently expired Premium subscription
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionRestorePremium {
}

/// Suggests the user to subscribe to the Premium subscription with annual payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionSubscribeToAnnualPremium {
}

/// Suggests the user to gift Telegram Premium to friends for Christmas
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionGiftPremiumForChristmas {
}

/// Suggests the user to extend their expiring Telegram Premium subscription
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionExtendPremium {
    /// A URL for managing Telegram Premium subscription
    pub manage_premium_subscription_url: String,
}

/// The bar for managing business bot was changed in a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatBusinessBotManageBar {
    /// Chat identifier
    pub chat_id: i64,
    /// The new value of the business bot manage bar; may be null
    pub business_bot_manage_bar: Option<crate::types::BusinessBotManageBar>,
}

/// A business connection has changed; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateBusinessConnection {
    /// New data about the connection
    pub connection: crate::types::BusinessConnection,
}

/// A new message was added to a business account; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewBusinessMessage {
    /// Unique identifier of the business connection
    pub connection_id: String,
    /// The new message
    pub message: crate::types::BusinessMessage,
}

/// A message in a business account was edited; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateBusinessMessageEdited {
    /// Unique identifier of the business connection
    pub connection_id: String,
    /// The edited message
    pub message: crate::types::BusinessMessage,
}

/// Messages in a business account were deleted; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateBusinessMessagesDeleted {
    /// Unique identifier of the business connection
    pub connection_id: String,
    /// Identifier of a chat in the business account in which messages were deleted
    pub chat_id: i64,
    /// Unique message identifiers of the deleted messages
    pub message_ids: Vec<i64>,
}

/// A new incoming callback query from a business message; for bots only
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewBusinessCallbackQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// Unique identifier of the business connection
    pub connection_id: String,
    /// The message from the business account from which the query originated
    pub message: crate::types::BusinessMessage,
    /// An identifier uniquely corresponding to the chat a message was sent to
    #[serde_as(as = "DisplayFromStr")]
    pub chat_instance: i64,
    /// Query payload
    pub payload: crate::enums::CallbackQueryPayload,
}

/// A chat boost has changed; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatBoost {
    /// Chat identifier
    pub chat_id: i64,
    /// New information about the boost
    pub boost: crate::types::ChatBoost,
}


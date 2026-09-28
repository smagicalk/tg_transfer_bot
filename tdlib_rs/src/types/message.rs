//!
//! TDLib `message` domain types.
//!
//! Types, enums, and functions for message content, rich formatting, reactions, and drafts.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A digit-only authentication code is delivered via a private Telegram message, which can be viewed from another active session
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeTelegramMessage {
    /// Length of the code
    pub length: i32,
}

/// Represents a part of the text that needs to be formatted in some unusual way
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TextEntity {
    /// Offset of the entity, in UTF-16 code units
    pub offset: i32,
    /// Length of the entity, in UTF-16 code units
    pub length: i32,
    /// Type of the entity
    #[serde(rename = "type")]
    pub r#type: crate::enums::TextEntityType,
}

/// Contains a list of text entities
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntities {
    /// List of text entities
    pub entities: Vec<crate::types::TextEntity>,
}

/// A text with some entities
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FormattedText {
    /// The text
    pub text: String,
    /// Entities contained in the text. Entities can be nested, but must not mutually intersect with each other.
    /// Pre, Code, PreCode, and DateTime entities can't contain other entities. BlockQuote entities can't contain other BlockQuote entities. Bold, Italic, Underline, Strikethrough, and Spoiler entities can contain and can be part of any other entities. All other entities can't contain each other
    pub entities: Vec<crate::types::TextEntity>,
}

/// Describes a message with rich formatting
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichMessage {
    /// Content of the message
    pub blocks: Vec<crate::enums::PageBlock>,
    /// True, if the message must be shown from right to left
    pub is_rtl: bool,
    /// True, if the object contains the full message. Otherwise, getFullRichMessage must be used to get the full message
    pub is_full: bool,
}

/// Describes a media to be used in a sent rich message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputRichMessageMedia {
    /// Unique identifier of the media; 1-64 base64url characters
    pub id: String,
    /// The media to send. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessagePhoto, inputMessageVideo, or inputMessageVoiceNote
    pub media: crate::enums::InputMessageContent,
}

/// A rich message defined by blocks
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichMessageSourceBlocks {
    /// Content of the message
    pub blocks: Vec<crate::enums::InputPageBlock>,
}

/// A Markdown-formatted rich message; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichMessageSourceMarkdown {
    /// Markdown-formatted text of the message
    pub text: String,
    /// Media used in the message
    pub media: Vec<crate::types::InputRichMessageMedia>,
}

/// An HTML-formatted rich message; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichMessageSourceHtml {
    /// HTML-formatted text of the message
    pub text: String,
    /// Media used in the message
    pub media: Vec<crate::types::InputRichMessageMedia>,
}

/// A rich message to send. Total length of all texts, including custom emoji alternative text and formula source, must not exceed getOption("rich_message_text_length_max").
/// The total number of all blocks, list items and table rows must not exceed getOption("rich_message_block_count_max").
/// The maximum allowed depth of nested blocks and rich texts is getOption("rich_message_depth_max").
/// The total number of media in all blocks must not exceed getOption("rich_message_media_count_max").
/// The maximum allowed number of table columns is getOption("rich_message_table_column_count_max")
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputRichMessage {
    /// Source of the rich message
    pub source: crate::enums::RichMessageSource,
    /// Pass true if the message must be shown from right to left
    pub is_rtl: bool,
    /// Pass true to enable detection of URLs, email addresses and other automatic blocks
    pub detect_automatic_blocks: bool,
}

/// A text with some changes highlighted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DiffText {
    /// The text
    pub text: String,
    /// Entities describing changes in the text. Entities don't mutually intersect with each other
    pub entities: Vec<crate::types::DiffEntity>,
}

/// A text fixed using fixTextWithAi
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FixedText {
    /// The resulting text
    pub text: crate::types::FormattedText,
    /// Changes made to the original text
    pub diff_text: crate::types::DiffText,
}

/// Contains an example of text composition style usage
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextCompositionStyleExample {
    /// Source text
    pub source_text: crate::types::FormattedText,
    /// The text after the style was applied to the source text
    pub result_text: crate::types::FormattedText,
}

/// Describes a style that can be used to compose a text
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextCompositionStyle {
    /// Name of the style
    pub name: String,
    /// Identifier of the custom emoji corresponding to the style; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
    /// Title of the style in the user application's language
    pub title: String,
    /// True, if the style is created by a user
    pub is_custom: bool,
    /// True, if the user is creator of the style
    pub is_creator: bool,
    /// Number of users that installed the style; for created custom styles only; 0 if unknown
    pub install_count: i32,
    /// Prompt of the style; for created custom styles only
    pub prompt: String,
    /// User identifier of the creator of the style; 0 if none or unknown
    pub creator_user_id: i64,
    /// Example of the style usage in English; may be null if unknown
    pub english_example: Option<crate::types::TextCompositionStyleExample>,
}

/// The transaction is a sending of a paid reaction to a message in a channel chat by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelPaidReactionSend {
    /// Identifier of the channel chat
    pub chat_id: i64,
    /// Identifier of the reacted message; may be 0 or an identifier of a deleted message
    pub message_id: i64,
}

/// The transaction is a receiving of a paid reaction to a message by the channel chat; relevant for channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelPaidReactionReceive {
    /// Identifier of the user who added the paid reaction
    pub user_id: i64,
    /// Identifier of the reacted message; may be 0 or an identifier of a deleted message
    pub message_id: i64,
}

/// The transaction is a sending of a paid message; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidMessageSend {
    /// Identifier of the chat that received the payment
    pub chat_id: i64,
    /// Number of sent paid messages
    pub message_count: i32,
}

/// The transaction is a receiving of a paid message; relevant for regular users, supergroup and channel chats only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidMessageReceive {
    /// Identifier of the sender of the message
    pub sender_id: crate::enums::MessageSender,
    /// Number of received paid messages
    pub message_count: i32,
    /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars paid for message sending
    pub commission_per_mille: i32,
    /// The Telegram Star amount that was received by Telegram; can be negative for refunds
    pub commission_star_amount: crate::types::StarAmount,
}

/// The message was sent by a known user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSenderUser {
    /// Identifier of the user who sent the message
    pub user_id: i64,
}

/// The message was sent on behalf of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSenderChat {
    /// Identifier of the chat that sent the message
    pub chat_id: i64,
}

/// Represents a list of message senders
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSenders {
    /// Approximate total number of message senders found
    pub total_count: i32,
    /// List of message senders
    pub senders: Vec<crate::enums::MessageSender>,
}

/// Represents a message sender, which can be used to send messages in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatMessageSender {
    /// The message sender
    pub sender: crate::enums::MessageSender,
    /// True, if Telegram Premium is needed to use the message sender
    pub needs_premium: bool,
}

/// Represents a list of message senders, which can be used to send messages in a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMessageSenders {
    /// List of available message senders
    pub senders: Vec<crate::types::ChatMessageSender>,
}

/// Contains read date of the message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReadDateRead {
    /// Point in time (Unix timestamp) when the message was read by the other user
    pub read_date: i32,
}

/// The message is unread yet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReadDateUnread {
}

/// The message is too old to get read date
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReadDateTooOld {
}

/// The read date is unknown due to privacy settings of the other user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReadDateUserPrivacyRestricted {
}

/// The read date is unknown due to privacy settings of the current user, but will be known if the user subscribes to Telegram Premium
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReadDateMyPrivacyRestricted {
}

/// Represents a viewer of a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageViewer {
    /// User identifier of the viewer
    pub user_id: i64,
    /// Approximate point in time (Unix timestamp) when the message was viewed
    pub view_date: i32,
}

/// Represents a list of message viewers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageViewers {
    /// List of message viewers
    pub viewers: Vec<crate::types::MessageViewer>,
}

/// The message was originally sent by a known user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginUser {
    /// Identifier of the user who originally sent the message
    pub sender_user_id: i64,
}

/// The message was originally sent by a user who is hidden by their privacy settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginHiddenUser {
    /// Name of the sender
    pub sender_name: String,
}

/// The message was originally sent on behalf of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginChat {
    /// Identifier of the chat that originally sent the message
    pub sender_chat_id: i64,
    /// For messages originally sent by an anonymous chat administrator, original message author signature
    pub author_signature: String,
}

/// The message was originally a post in a channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginChannel {
    /// Identifier of the channel chat to which the message was originally sent
    pub chat_id: i64,
    /// Message identifier of the original message
    pub message_id: i64,
    /// Original post author signature
    pub author_signature: String,
}

/// The paid reaction in a channel chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionTypePaid {
}

/// A paid reaction on behalf of the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidReactionTypeRegular {
}

/// An anonymous paid reaction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidReactionTypeAnonymous {
}

/// A paid reaction on behalf of an owned chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidReactionTypeChat {
    /// Identifier of the chat
    pub chat_id: i64,
}

/// Contains information about a forwarded message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageForwardInfo {
    /// Origin of the forwarded message
    pub origin: crate::enums::MessageOrigin,
    /// Point in time (Unix timestamp) when the message was originally sent
    pub date: i32,
    /// For messages forwarded to the chat with the current user (Saved Messages), to the Replies bot chat, or to the channel's discussion group, information about the source message from which the message was forwarded last time; may be null for other forwards or if unknown
    pub source: Option<crate::types::ForwardSource>,
    /// The type of public service announcement for the forwarded message
    pub public_service_announcement_type: String,
}

/// Contains information about a message created with importMessages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageImportInfo {
    /// Name of the original sender
    pub sender_name: String,
    /// Point in time (Unix timestamp) when the message was originally sent
    pub date: i32,
}

/// Contains information about replies to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReplyInfo {
    /// Number of times the message was directly or indirectly replied
    pub reply_count: i32,
    /// Identifiers of at most 3 recent repliers to the message; available in channels with a discussion supergroup. The users and chats are expected to be inaccessible: only their photo and name will be available
    pub recent_replier_ids: Vec<crate::enums::MessageSender>,
    /// Identifier of the last read incoming reply to the message
    pub last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing reply to the message
    pub last_read_outbox_message_id: i64,
    /// Identifier of the last reply to the message
    pub last_message_id: i64,
}

/// Contains information about a reaction to a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageReaction {
    /// Type of the reaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::ReactionType,
    /// Number of times the reaction was added
    pub total_count: i32,
    /// True, if the reaction is chosen by the current user
    pub is_chosen: bool,
    /// Identifier of the message sender used by the current user to add the reaction; may be null if unknown or the reaction isn't chosen
    pub used_sender_id: Option<crate::enums::MessageSender>,
    /// Identifiers of at most 3 recent message senders, added the reaction; available in private, basic group and supergroup chats
    pub recent_sender_ids: Vec<crate::enums::MessageSender>,
}

/// Contains a list of reactions added to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReactions {
    /// List of added reactions
    pub reactions: Vec<crate::types::MessageReaction>,
    /// True, if the reactions are tags and Telegram Premium users can filter messages by them
    pub are_tags: bool,
    /// Information about top users that added the paid reaction
    pub paid_reactors: Vec<crate::types::PaidReactor>,
    /// True, if the list of added reactions is available using getMessageAddedReactions
    pub can_get_added_reactions: bool,
}

/// Contains information about interactions with a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageInteractionInfo {
    /// Number of times the message was viewed
    pub view_count: i32,
    /// Number of times the message was forwarded
    pub forward_count: i32,
    /// Information about direct or indirect replies to the message; may be null. Currently, available only in channels with a discussion supergroup and discussion supergroups for messages, which are not replies itself
    pub reply_info: Option<crate::types::MessageReplyInfo>,
    /// The list of reactions or tags added to the message; may be null
    pub reactions: Option<crate::types::MessageReactions>,
}

/// Contains information about an unread reaction to a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnreadReaction {
    /// Type of the reaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::ReactionType,
    /// Identifier of the sender, added the reaction
    pub sender_id: crate::enums::MessageSender,
    /// True, if the reaction was added with a big animation
    pub is_big: bool,
}

/// Contains information about an effect added to a message
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageEffect {
    /// Unique identifier of the effect
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Static icon for the effect in WEBP format; may be null if none
    pub static_icon: Option<crate::types::Sticker>,
    /// Emoji corresponding to the effect that can be used if static icon isn't available
    pub emoji: String,
    /// True, if Telegram Premium subscription is required to use the effect
    pub is_premium: bool,
    /// Type of the effect
    #[serde(rename = "type")]
    pub r#type: crate::enums::MessageEffectType,
}

/// The message is being sent now, but has not yet been delivered to the server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSendingStatePending {
    /// Non-persistent message sending identifier, specified by the application
    pub sending_id: i32,
}

/// The message failed to be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSendingStateFailed {
    /// The cause of the message sending failure
    pub error: crate::types::Error,
    /// True, if the message can be re-sent using resendMessages or readdQuickReplyShortcutMessages
    pub can_retry: bool,
    /// True, if the message can be re-sent only on behalf of a different sender
    pub need_another_sender: bool,
    /// True, if the message can be re-sent only if another quote is chosen in the message that is replied by the given message
    pub need_another_reply_quote: bool,
    /// True, if the message can be re-sent only if the message to be replied is removed. This will be done automatically by resendMessages
    pub need_drop_reply: bool,
    /// The number of Telegram Stars that must be paid to send the message; 0 if the current amount is correct
    pub required_paid_message_star_count: i64,
    /// Time left before the message can be re-sent, in seconds. No update is sent when this field changes
    pub retry_after: f64,
}

/// Describes manually or automatically chosen quote from another message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextQuote {
    /// Text of the quote. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities can be present in the text
    pub text: crate::types::FormattedText,
    /// Approximate quote position in the original message in UTF-16 code units as specified by the message sender
    pub position: i32,
    /// True, if the quote was manually chosen by the message sender
    pub is_manual: bool,
}

/// Describes manually chosen quote from another message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputTextQuote {
    /// Text of the quote; 0-getOption("message_reply_quote_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed to be kept and must be kept in the quote
    pub text: crate::types::FormattedText,
    /// Quote position in the original message in UTF-16 code units
    pub position: i32,
}

/// Describes a message replied by a given message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReplyToMessage {
    /// The identifier of the chat to which the message belongs; may be 0 if the replied message is in unknown chat
    pub chat_id: i64,
    /// The identifier of the message; may be 0 if the replied message is in unknown chat
    pub message_id: i64,
    /// Chosen quote from the replied message; may be null if none
    pub quote: Option<crate::types::TextQuote>,
    /// Identifier of the checklist task in the original message that was replied; 0 if none
    pub checklist_task_id: i32,
    /// Identifier of the poll option in the original message that was replied; empty if none
    pub poll_option_id: String,
    /// Information about origin of the message if the message was from another chat or topic; may be null for messages from the same chat
    pub origin: Option<crate::enums::MessageOrigin>,
    /// Point in time (Unix timestamp) when the message was sent if the message was from another chat or topic; 0 for messages from the same chat
    pub origin_send_date: i32,
    /// Media content of the message if the message was from another chat or topic; may be null for messages from the same chat and messages without media.
    /// Can be only one of the following types: messageAnimation, messageAudio, messageChecklist, messageContact, messageDice, messageDocument, messageGame,
    /// messageGiveaway, messageGiveawayWinners, messageInvoice, messageLocation, messagePaidMedia, messagePhoto, messagePoll, messageStakeDice,
    /// messageSticker, messageStory, messageText (for link preview), messageVenue, messageVideo, messageVideoNote, or messageVoiceNote
    pub content: Option<crate::enums::MessageContent>,
}

/// Describes a message to be replied in the same chat and forum topic
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageReplyToMessage {
    /// The identifier of the message to be replied in the same chat and forum topic. A message can be replied in the same chat and forum topic only if messageProperties.can_be_replied
    pub message_id: i64,
    /// Quote from the message to be replied; pass null if none. Must always be null for replies in secret chats
    pub quote: Option<crate::types::InputTextQuote>,
    /// Identifier of the checklist task in the message to be replied; pass 0 to reply to the whole message
    pub checklist_task_id: i32,
    /// Identifier of the poll option in the message to be replied; pass an empty string if none
    pub poll_option_id: String,
}

/// Describes a message to be replied that is from a different chat or a forum topic; not supported in secret chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageReplyToExternalMessage {
    /// The identifier of the chat to which the message to be replied belongs
    pub chat_id: i64,
    /// The identifier of the message to be replied in the specified chat. A message can be replied in another chat or forum topic only if messageProperties.can_be_replied_in_another_chat
    pub message_id: i64,
    /// Quote from the message to be replied; pass null if none
    pub quote: Option<crate::types::InputTextQuote>,
    /// Identifier of the checklist task in the message to be replied; pass 0 to reply to the whole message
    pub checklist_task_id: i32,
    /// Identifier of the poll option in the message to be replied; pass an empty string if none
    pub poll_option_id: String,
}

/// Describes an ephemeral message to be replied; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageReplyToEphemeralMessage {
    /// The identifier of the ephemeral message to be replied
    pub ephemeral_message_id: i32,
}

/// Describes an ephemeral content of a regular message, which must be shown instead of the regular content
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EphemeralMessageContent {
    /// True, if content of the message can be saved locally
    pub can_be_saved: bool,
    /// True, if media timestamp entities refers to a media in this message as opposed to a media in the replied message
    pub has_timestamped_media: bool,
    /// Content of the message
    pub content: crate::enums::MessageContent,
    /// Reply markup for the message; may be null if none
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
}

/// Describes a message
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Message {
    /// Message identifier; unique for the chat to which the message belongs
    pub id: i64,
    /// Identifier of the sender of the message
    pub sender_id: crate::enums::MessageSender,
    /// Identifier of the user or the chat which received the ephemeral message; may be null. Always null for non-ephemeral messages
    pub receiver_id: Option<crate::enums::MessageSender>,
    /// Chat identifier
    pub chat_id: i64,
    /// The sending state of the message; may be null if the message isn't being sent and didn't fail to be sent
    pub sending_state: Option<crate::enums::MessageSendingState>,
    /// The scheduling state of the message; may be null if the message isn't scheduled
    pub scheduling_state: Option<crate::enums::MessageSchedulingState>,
    /// True, if the message is outgoing
    pub is_outgoing: bool,
    /// True, if the message is pinned
    pub is_pinned: bool,
    /// True, if the message was sent because of a scheduled action by the message sender, for example, as away, or greeting service message
    pub is_from_offline: bool,
    /// True, if content of the message can be saved locally
    pub can_be_saved: bool,
    /// True, if media timestamp entities refers to a media in this message as opposed to a media in the replied message
    pub has_timestamped_media: bool,
    /// True, if the message is a channel post. All messages to channels are channel posts, all other messages are not channel posts
    pub is_channel_post: bool,
    /// True, if the message is a suggested channel post which was paid in Telegram Stars; a warning must be shown if the message is deleted in less than getOption("suggested_post_lifetime_min") seconds after sending
    pub is_paid_star_suggested_post: bool,
    /// True, if the message is a suggested channel post which was paid in TON Grams; a warning must be shown if the message is deleted in less than getOption("suggested_post_lifetime_min") seconds after sending
    pub is_paid_gram_suggested_post: bool,
    /// True, if the message contains an unread mention for the current user
    pub contains_unread_mention: bool,
    /// True, if the message is a poll message with unread votes
    pub contains_unread_poll_votes: bool,
    /// Point in time (Unix timestamp) when the message was sent; 0 for scheduled messages
    pub date: i32,
    /// Point in time (Unix timestamp) when the message was last edited; 0 for scheduled messages. If getOption("show_message_edit_date_by_default") is true,
    /// then the date must be shown along with the message instead of the date when the message was sent
    pub edit_date: i32,
    /// Information about the initial message sender; may be null if none or unknown
    pub forward_info: Option<crate::types::MessageForwardInfo>,
    /// Information about the initial message for messages created with importMessages; may be null if the message isn't imported
    pub import_info: Option<crate::types::MessageImportInfo>,
    /// Information about interactions with the message; may be null if none
    pub interaction_info: Option<crate::types::MessageInteractionInfo>,
    /// Information about unread reactions added to the message
    pub unread_reactions: Vec<crate::types::UnreadReaction>,
    /// Information about fact-check added to the message; may be null if none
    pub fact_check: Option<crate::types::FactCheck>,
    /// Information about the suggested post; may be null if the message isn't a suggested post
    pub suggested_post_info: Option<crate::types::SuggestedPostInfo>,
    /// Information about the message or the story this message is replying to; may be null if none
    pub reply_to: Option<crate::enums::MessageReplyTo>,
    /// Identifier of the topic within the chat to which the message belongs; may be null if none; may change when the chat is converted to a forum or back
    pub topic_id: Option<crate::enums::MessageTopic>,
    /// The message's self-destruct type; may be null if none
    pub self_destruct_type: Option<crate::enums::MessageSelfDestructType>,
    /// Time left before the message self-destruct timer expires, in seconds; 0 if self-destruction isn't scheduled yet
    pub self_destruct_in: f64,
    /// Time left before the message will be automatically deleted by message_auto_delete_time setting of the chat, in seconds; 0 if never
    pub auto_delete_in: f64,
    /// If non-zero, the user identifier of the inline bot through which this message was sent
    pub via_bot_user_id: i64,
    /// The identifier of the user or chat which used a guest bot to send the message; may be null if none
    pub guest_bot_caller_id: Option<crate::enums::MessageSender>,
    /// If non-zero, the user identifier of the business bot that sent this message
    pub sender_business_bot_user_id: i64,
    /// Number of times the sender of the message boosted the supergroup at the time the message was sent; 0 if none or unknown. For messages sent by the current user, supergroupFullInfo.my_boost_count must be used instead
    pub sender_boost_count: i32,
    /// Tag of the sender of the message in the supergroup at the time the message was sent; may be empty if none or unknown. For messages sent in basic groups or supergroup administrators, the current custom title or tag must be used instead
    pub sender_tag: String,
    /// The number of Telegram Stars the sender paid to send the message
    pub paid_message_star_count: i64,
    /// For channel posts and anonymous group messages, optional author signature
    pub author_signature: String,
    /// Unique identifier of an album this message belongs to; 0 if none. Only audios, documents, photos and videos can be grouped together in albums
    #[serde_as(as = "DisplayFromStr")]
    pub media_album_id: i64,
    /// Unique identifier of the effect added to the message; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub effect_id: i64,
    /// Information about the restrictions that must be applied to the message content; may be null if none
    pub restriction_info: Option<crate::types::RestrictionInfo>,
    /// IETF language tag of the message language on which it can be summarized; empty if summary isn't available for the message
    pub summary_language_code: String,
    /// Content of the message
    pub content: crate::enums::MessageContent,
    /// Content of the message, which is visible only to the current user and must be shown instead of the regular content; may be null if none
    pub ephemeral_content: Option<crate::types::EphemeralMessageContent>,
    /// Reply markup for the message; may be null if none
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
    /// Unique identifier of the ephemeral message if the message is ephemeral; for bots only
    pub ephemeral_message_id: i32,
    /// Identifier that uniquely corresponds to the chat to which the message was sent; for bots only
    #[serde_as(as = "DisplayFromStr")]
    pub chat_instance: i64,
}

/// Contains a list of messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Messages {
    /// Approximate total number of messages found
    pub total_count: i32,
    /// List of messages; messages may be null
    pub messages: Vec<Option<crate::types::Message>>,
}

/// Contains a list of messages found by a search
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundMessages {
    /// Approximate total number of messages found; -1 if unknown
    pub total_count: i32,
    /// List of messages
    pub messages: Vec<crate::types::Message>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Contains a list of messages found by a search in a given chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundChatMessages {
    /// Approximate total number of messages found; -1 if unknown
    pub total_count: i32,
    /// List of messages
    pub messages: Vec<crate::types::Message>,
    /// The offset for the next request. If 0, there are no more results
    pub next_from_message_id: i64,
}

/// Contains information about a message in a specific position
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePosition {
    /// 0-based message position in the full list of suitable messages
    pub position: i32,
    /// Message identifier
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the message was sent
    pub date: i32,
}

/// Contains a list of message positions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePositions {
    /// Total number of messages found
    pub total_count: i32,
    /// List of message positions
    pub positions: Vec<crate::types::MessagePosition>,
}

/// Contains information about found messages sent on a specific day
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageCalendarDay {
    /// Total number of found messages sent on the day
    pub total_count: i32,
    /// First message sent on the day
    pub message: crate::types::Message,
}

/// Contains information about found messages, split by days according to the option "utc_time_offset"
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageCalendar {
    /// Total number of found messages
    pub total_count: i32,
    /// Information about messages sent
    pub days: Vec<crate::types::MessageCalendarDay>,
}

/// The message is from a chat list or a forum topic list
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceChatList {
}

/// The message is from search results, including file downloads, local file list, outgoing document messages, calendar
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceSearch {
}

/// The message is from a chat event log
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceChatEventLog {
}

/// The message was screenshotted; the source must be used only if the message content was visible during the screenshot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceScreenshot {
}

/// The message is from some other source
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceOther {
}

/// Describes a sponsored message
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SponsoredMessage {
    /// Message identifier; unique for the chat to which the sponsored message belongs among both ordinary and sponsored messages
    pub message_id: i64,
    /// True, if the message needs to be labeled as "recommended" instead of "sponsored"
    pub is_recommended: bool,
    /// True, if the message can be reported to Telegram moderators through reportChatSponsoredMessage
    pub can_be_reported: bool,
    /// Content of the message. Currently, can be only of the types messageText, messageAnimation, messagePhoto, or messageVideo. Video messages can be viewed fullscreen.
    /// The content must be fully downloaded before the message is shown
    pub content: crate::enums::MessageContent,
    /// Information about the sponsor of the message
    pub sponsor: crate::types::AdvertisementSponsor,
    /// Title of the sponsored message
    pub title: String,
    /// Text for the message action button
    pub button_text: String,
    /// Identifier of the accent color for title, button text and message background
    pub accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the message background; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub background_custom_emoji_id: i64,
    /// If non-empty, additional information about the sponsored message to be shown along with the message
    pub additional_info: String,
}

/// Contains a list of sponsored messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SponsoredMessages {
    /// List of sponsored messages
    pub messages: Vec<crate::types::SponsoredMessage>,
    /// The minimum number of messages between shown sponsored messages, or 0 if only one sponsored message must be shown after all ordinary messages
    pub messages_between: i32,
}

/// A text message draft
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DraftMessageContentText {
    /// Formatted text to be saved as a draft; 0-getOption("message_text_length_max") characters
    pub text: crate::types::FormattedText,
    /// Options to be used for generation of a link preview; may be null if none; pass null to use default link preview options
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
}

/// A rich message draft; not supported in setChatDraftMessage
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DraftMessageContentRichMessage {
    /// The rich message
    pub message: crate::types::RichMessage,
}

/// A rich message draft; only for setChatDraftMessage
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DraftMessageContentInputRichMessage {
    /// The rich message
    pub message: crate::types::InputRichMessage,
}

/// A voice note message draft
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DraftMessageContentVoiceNote {
    /// Path to the file with the voice note
    pub file_path: String,
    /// Duration of the voice note, in seconds
    pub duration: i32,
    /// Waveform representation of the voice note in 5-bit format
    pub waveform: String,
    /// Voice note self-destruct type; may be null if none; pass null if none; private chats only
    pub self_destruct_type: Option<crate::enums::MessageSelfDestructType>,
}

/// Contains information about a message draft
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DraftMessage {
    /// Information about the message to be replied; inputMessageReplyToStory is unsupported; may be null if none
    pub reply_to: Option<crate::enums::InputMessageReplyTo>,
    /// Point in time (Unix timestamp) when the draft was created
    pub date: i32,
    /// Content of the message draft
    pub content: crate::enums::DraftMessageContent,
    /// Identifier of the effect to apply to the message when it is sent; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub effect_id: i64,
    /// Information about the suggested post; may be null if none
    pub suggested_post_info: Option<crate::types::InputSuggestedPostInfo>,
}

/// All reactions are available in the chat, excluding the paid reaction and custom reactions in channel chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAvailableReactionsAll {
    /// The maximum allowed number of reactions per message; 1-11
    pub max_reaction_count: i32,
}

/// Only specific reactions are available in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAvailableReactionsSome {
    /// The list of reactions
    pub reactions: Vec<crate::enums::ReactionType>,
    /// The maximum allowed number of reactions per message; 1-11
    pub max_reaction_count: i32,
}

/// Represents a tag used in Saved Messages or a Saved Messages topic
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTag {
    /// The tag
    pub tag: crate::enums::ReactionType,
    /// Label of the tag; 0-12 characters. Always empty if the tag is returned for a Saved Messages topic
    pub label: String,
    /// Number of times the tag was used; may be 0 if the tag has non-empty label
    pub count: i32,
}

/// Contains a list of tags used in Saved Messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTags {
    /// List of tags
    pub tags: Vec<crate::types::SavedMessagesTag>,
}

/// A simple button, with text that must be sent when the button is pressed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeText {
}

/// A button that copies specified text to clipboard
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeCopyText {
    /// The text to copy to clipboard
    pub text: String,
}

/// The button is from a bot's message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonSourceMessage {
    /// Identifier of the chat with the message
    pub chat_id: i64,
    /// Identifier of the message with the button
    pub message_id: i64,
}

/// Contains information about a message thread
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageThreadInfo {
    /// Identifier of the chat to which the message thread belongs
    pub chat_id: i64,
    /// Message thread identifier, unique within the chat
    pub message_thread_id: i64,
    /// Information about the message thread; may be null for forum topic threads
    pub reply_info: Option<crate::types::MessageReplyInfo>,
    /// Approximate number of unread messages in the message thread
    pub unread_message_count: i32,
    /// The messages from which the thread starts. The messages are returned in reverse chronological order (i.e., in order of decreasing message_id)
    pub messages: Vec<crate::types::Message>,
    /// A draft of a message in the message thread; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
}

/// A plain text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichTextPlain {
    /// Text
    pub text: String,
}

/// A bold rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextBold {
    /// Text
    pub text: crate::enums::RichText,
}

/// An italicized rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextItalic {
    /// Text
    pub text: crate::enums::RichText,
}

/// An underlined rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextUnderline {
    /// Text
    pub text: crate::enums::RichText,
}

/// A strikethrough rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextStrikethrough {
    /// Text
    pub text: crate::enums::RichText,
}

/// A spoilered rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextSpoiler {
    /// Text
    pub text: crate::enums::RichText,
}

/// A subscript rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextSubscript {
    /// Text
    pub text: crate::enums::RichText,
}

/// A superscript rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextSuperscript {
    /// Text
    pub text: crate::enums::RichText,
}

/// A marked rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextMarked {
    /// Text
    pub text: crate::enums::RichText,
}

/// A date and time
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextDateTime {
    /// Original text
    pub text: crate::enums::RichText,
    /// Point in time (Unix timestamp) representing the date and time
    pub unix_time: i32,
    /// Date and time formatting type; may be null if none and the original text must not be changed
    pub formatting_type: Option<crate::enums::DateTimeFormattingType>,
}

/// A mention of a Telegram user or chat by a username
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextMention {
    /// Text
    pub text: crate::enums::RichText,
    /// The username
    pub username: String,
}

/// A hashtag
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextHashtag {
    /// Text
    pub text: crate::enums::RichText,
    /// The hashtag
    pub hashtag: String,
}

/// A cashtag
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextCashtag {
    /// Text
    pub text: crate::enums::RichText,
    /// The cashtag
    pub cashtag: String,
}

/// A bank card number
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextBankCardNumber {
    /// Text
    pub text: crate::enums::RichText,
    /// The number of the bank card
    pub bank_card_number: String,
}

/// A fixed-width rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextFixed {
    /// Text
    pub text: crate::enums::RichText,
}

/// A rich text that serves as a mention of a user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextMentionName {
    /// Text
    pub text: crate::enums::RichText,
    /// Identifier of the mentioned user
    pub user_id: i64,
}

/// A rich text URL link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextUrl {
    /// Text
    pub text: crate::enums::RichText,
    /// URL
    pub url: String,
    /// True, if the URL has cached instant view server-side; instant view only
    pub is_cached: bool,
}

/// A rich text email address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextEmailAddress {
    /// Text
    pub text: crate::enums::RichText,
    /// Email address
    pub email_address: String,
}

/// A rich text phone number
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextPhoneNumber {
    /// Text
    pub text: crate::enums::RichText,
    /// Phone number
    pub phone_number: String,
}

/// A small image inside the text; instant view only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextIcon {
    /// The image represented as a document. The image can be in GIF, JPEG or PNG format
    pub document: crate::types::Document,
    /// Width of a bounding box in which the image must be shown; 0 if unknown
    pub width: i32,
    /// Height of a bounding box in which the image must be shown; 0 if unknown
    pub height: i32,
}

/// A mathematical expression
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichTextMathematicalExpression {
    /// The expression in LaTeX format
    pub expression: String,
}

/// A button
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextButton {
    /// The button
    pub button: crate::types::InlineButton,
}

/// A rich text replacing another rich text; not supported in inputRichMessage
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextDiff {
    /// Text
    pub text: crate::enums::RichText,
    /// The old text
    pub old_text: crate::enums::RichText,
}

/// A reference
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextReference {
    /// Reference name
    pub name: String,
    /// Text of the reference
    pub text: crate::enums::RichText,
}

/// A link to a reference on the same page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextReferenceLink {
    /// The link text
    pub text: crate::enums::RichText,
    /// The reference name
    pub reference_name: String,
    /// An HTTP URL that opens the reference
    pub url: String,
}

/// An anchor
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichTextAnchor {
    /// Anchor name
    pub name: String,
}

/// A link to an anchor on the same page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextAnchorLink {
    /// The link text
    pub text: crate::enums::RichText,
    /// The anchor name. If the name is empty, the link must bring back to top
    pub anchor_name: String,
    /// An HTTP URL that opens the anchor
    pub url: String,
}

/// A concatenation of rich texts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichTexts {
    /// Texts
    pub texts: Vec<crate::enums::RichText>,
}

/// Contains a caption of another block
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockCaption {
    /// Content of the caption
    pub text: crate::enums::RichText,
    /// Block credit (like HTML tag <cite>); may be null if none
    pub credit: Option<crate::enums::RichText>,
}

/// The link is a link to a direct messages chat of a channel
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeDirectMessagesChat {
    /// Photo of the channel chat; may be null
    pub photo: Option<crate::types::ChatPhoto>,
}

/// The link is a link to a text or a poll Telegram message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeMessage {
}

/// The link is a link to a text composition style
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeTextCompositionStyle {
    /// Identifier of the custom emoji corresponding to the style; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}

/// A text message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageText {
    /// Text of the message
    pub text: crate::types::FormattedText,
    /// A link preview attached to the message; may be null
    pub link_preview: Option<crate::types::LinkPreview>,
    /// Options which were used for generation of the link preview; may be null if default options were used
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
}

/// A rich message; the message can have multiple media of the same type, all of which must be shown in the corresponding profile tab
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageRichMessage {
    /// The rich message
    pub message: crate::types::RichMessage,
}

/// A message with paid media
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaidMedia {
    /// Number of Telegram Stars needed to buy access to the media in the message
    pub star_count: i64,
    /// Information about the media
    pub media: Vec<crate::enums::PaidMedia>,
    /// Media caption
    pub caption: crate::types::FormattedText,
    /// True, if the caption must be shown above the media; otherwise, the caption must be shown below the media
    pub show_caption_above_media: bool,
}

/// A voice note message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageVoiceNote {
    /// The voice note description
    pub voice_note: crate::types::VoiceNote,
    /// Voice note caption
    pub caption: crate::types::FormattedText,
    /// True, if at least one of the recipients has listened to the voice note
    pub is_listened: bool,
}

/// A self-destructed voice note message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageExpiredVoiceNote {
}

/// A message with a live location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageLiveLocation {
    /// The current location
    pub location: crate::types::LiveLocation,
    /// Left time for which the location can be updated, in seconds. If 0, then the location can't be updated anymore. The update updateMessageContent is not sent when this field changes
    pub expires_in: i32,
}

/// A message with a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageLocation {
    /// The location
    pub location: crate::types::Location,
}

/// A message with information about a venue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageVenue {
    /// The venue description
    pub venue: crate::types::Venue,
}

/// A message with a user contact
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageContact {
    /// The contact description
    pub contact: crate::types::Contact,
}

/// A message with a game
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGame {
    /// The game description
    pub game: crate::types::Game,
}

/// A message with a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChecklist {
    /// The checklist description
    pub list: crate::types::Checklist,
}

/// A newly created basic group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageBasicGroupChatCreate {
    /// Title of the basic group
    pub title: String,
    /// User identifiers of members in the basic group
    pub member_user_ids: Vec<i64>,
}

/// A newly created supergroup or channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSupergroupChatCreate {
    /// Title of the supergroup or channel
    pub title: String,
}

/// An updated chat title
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatChangeTitle {
    /// New chat title
    pub title: String,
}

/// The owner of the chat has left
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatOwnerLeft {
    /// Identifier of the user who will become the new owner of the chat if the previous owner isn't return; 0 if none
    pub new_owner_user_id: i64,
}

/// The owner of the chat has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatOwnerChanged {
    /// Identifier of the user who is the new owner of the chat
    pub new_owner_user_id: i64,
}

/// Chat has_protected_content setting was changed or request to change it was rejected
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatHasProtectedContentToggled {
    /// Identifier of the message with the request to change the setting; can be an identifier of a deleted message or 0
    pub request_message_id: i64,
    /// Previous value of the setting
    pub old_has_protected_content: bool,
    /// New value of the setting
    pub new_has_protected_content: bool,
}

/// Chat has_protected_content setting was requested to be disabled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatHasProtectedContentDisableRequested {
    /// True, if the request has expired
    pub is_expired: bool,
}

/// New chat members were added
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatAddMembers {
    /// User identifiers of the new members
    pub member_user_ids: Vec<i64>,
}

/// A new member joined the chat via an invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatJoinByLink {
}

/// A new member was accepted to the chat by an administrator
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatJoinByRequest {
}

/// A new member joined the chat from a community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatJoinFromCommunity {
    /// Identifier of the community from which the user joined the chat
    pub community_id: i64,
}

/// A chat member was deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatDeleteMember {
    /// User identifier of the deleted chat member
    pub user_id: i64,
}

/// The chat was added to a community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatAddedToCommunity {
    /// Identifier of the community to which the chat was added
    pub community_id: i64,
}

/// The chat was removed from a community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatRemovedFromCommunity {
}

/// A basic group was upgraded to a supergroup and was deactivated as the result
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatUpgradeTo {
    /// Identifier of the supergroup to which the basic group was upgraded
    pub supergroup_id: i64,
}

/// A supergroup has been created from a basic group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatUpgradeFrom {
    /// Title of the newly created supergroup
    pub title: String,
    /// The identifier of the original basic group
    pub basic_group_id: i64,
}

/// A message has been pinned
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePinMessage {
    /// Identifier of the pinned message, can be an identifier of a deleted message or 0
    pub message_id: i64,
}

/// A screenshot of a message in the chat has been taken
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageScreenshotTaken {
}

/// A new background was set in the chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageChatSetBackground {
    /// Identifier of the message with a previously set same background; 0 if none. Can be an identifier of a deleted message
    pub old_background_message_id: i64,
    /// The new background
    pub background: crate::types::ChatBackground,
    /// True, if the background was set only for self
    pub only_for_self: bool,
}

/// A theme in the chat has been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatSetTheme {
    /// New theme for the chat; may be null if chat theme was reset to the default one
    pub theme: Option<crate::enums::ChatTheme>,
}

/// The auto-delete or self-destruct timer for messages in the chat has been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatSetMessageAutoDeleteTime {
    /// New value auto-delete or self-destruct time, in seconds; 0 if disabled
    pub message_auto_delete_time: i32,
    /// If not 0, a user identifier, which default setting was automatically applied
    pub from_user_id: i64,
}

/// A birthdate was suggested to be set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestBirthdate {
    /// The suggested birthdate. Use the method setBirthdate to apply the birthdate
    pub birthdate: crate::types::Birthdate,
}

/// A non-standard action has happened in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageCustomServiceAction {
    /// Message text to be shown in the chat
    pub text: String,
}

/// A new high score was achieved in a game
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageGameScore {
    /// Identifier of the message with the game, can be an identifier of a deleted message
    pub game_message_id: i64,
    /// Identifier of the game; may be different from the games presented in the message with the game
    #[serde_as(as = "DisplayFromStr")]
    pub game_id: i64,
    /// New score
    pub score: i32,
}

/// TON Grams were gifted to a user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiftedGrams {
    /// The identifier of a user who gifted Grams; 0 if the gift was anonymous or is outgoing
    pub gifter_user_id: i64,
    /// The identifier of a user who received Grams; 0 if the gift is incoming
    pub receiver_user_id: i64,
    /// The received Gram amount, in the smallest units of the cryptocurrency
    pub gram_amount: i64,
    /// Identifier of the transaction for Gram credit; for receiver only
    pub transaction_id: String,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}

/// A regular gift was received or sent by the current user, or the current user was notified about a channel gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGift {
    /// The gift
    pub gift: crate::types::Gift,
    /// Sender of the gift; may be null for outgoing messages about prepaid upgrade of gifts from unknown users
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Receiver of the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Unique identifier of the received gift for the current user; only for the receiver of the gift
    pub received_gift_id: String,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// Unique number of the gift among gifts upgraded from the same gift after upgrade; 0 if yet unassigned
    pub unique_gift_number: i32,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the regular gift; 0 if the gift can't be sold by the receiver
    pub sell_star_count: i64,
    /// Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift
    pub prepaid_upgrade_star_count: i64,
    /// True, if the upgrade was bought after the gift was sent. In this case, prepaid upgrade cost must not be added to the gift cost
    pub is_upgrade_separate: bool,
    /// True, if the message is a notification about a gift won on an auction
    pub is_from_auction: bool,
    /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: bool,
    /// True, if the gift is displayed on the user's or the channel's profile page; only for the receiver of the gift
    pub is_saved: bool,
    /// True, if the message is about prepaid upgrade of the gift by another user
    pub is_prepaid_upgrade: bool,
    /// True, if the gift can be upgraded to a unique gift; only for the receiver of the gift
    pub can_be_upgraded: bool,
    /// True, if the gift was converted to Telegram Stars; only for the receiver of the gift
    pub was_converted: bool,
    /// True, if the gift was upgraded to a unique gift
    pub was_upgraded: bool,
    /// True, if the gift was refunded and isn't available anymore
    pub was_refunded: bool,
    /// Identifier of the corresponding upgraded gift; may be empty if unknown. Use getReceivedGift to get information about the gift
    pub upgraded_received_gift_id: String,
    /// If non-empty, then the user can pay for an upgrade of the gift using buyGiftUpgrade
    pub prepaid_upgrade_hash: String,
}

/// An upgraded gift was received or sent by the current user, or the current user was notified about a channel gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageUpgradedGift {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// Sender of the gift; may be null for anonymous gifts
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Receiver of the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Origin of the upgraded gift
    pub origin: crate::enums::UpgradedGiftOrigin,
    /// Unique identifier of the received gift for the current user; only for the receiver of the gift
    pub received_gift_id: String,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: bool,
    /// True, if the gift is displayed on the user's or the channel's profile page; only for the receiver of the gift
    pub is_saved: bool,
    /// True, if the gift can be transferred to another owner; only for the receiver of the gift
    pub can_be_transferred: bool,
    /// True, if the gift has already been transferred to another owner; only for the receiver of the gift
    pub was_transferred: bool,
    /// Number of Telegram Stars that must be paid to transfer the upgraded gift; only for the receiver of the gift
    pub transfer_star_count: i64,
    /// Number of Telegram Stars that must be paid to drop original details of the upgraded gift; 0 if not available; only for the receiver of the gift
    pub drop_original_details_star_count: i64,
    /// Point in time (Unix timestamp) when the gift can be transferred to another owner; can be in the past; 0 if the gift can be transferred immediately or transfer isn't possible; only for the receiver of the gift
    pub next_transfer_date: i32,
    /// Point in time (Unix timestamp) when the gift can be resold to another user; can be in the past; 0 if the gift can't be resold; only for the receiver of the gift
    pub next_resale_date: i32,
    /// Point in time (Unix timestamp) when the gift can be transferred to the TON blockchain as an NFT; can be in the past; 0 if NFT export isn't possible; only for the receiver of the gift
    pub export_date: i32,
    /// Point in time (Unix timestamp) when the gift can be used to craft another gift; can be in the past; only for the receiver of the gift
    pub craft_date: i32,
}

/// A gift which purchase, upgrade or transfer were refunded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageRefundedUpgradedGift {
    /// The gift
    pub gift: crate::types::Gift,
    /// Sender of the gift
    pub sender_id: crate::enums::MessageSender,
    /// Receiver of the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Origin of the upgraded gift
    pub origin: crate::enums::UpgradedGiftOrigin,
}

/// An offer to purchase an upgraded gift was sent or received
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageUpgradedGiftPurchaseOffer {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// State of the offer
    pub state: crate::enums::GiftPurchaseOfferState,
    /// The proposed price
    pub price: crate::enums::GiftResalePrice,
    /// Point in time (Unix timestamp) when the offer will expire or has expired
    pub expiration_date: i32,
}

/// An offer to purchase a gift was rejected or expired
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageUpgradedGiftPurchaseOfferRejected {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// The proposed price
    pub price: crate::enums::GiftResalePrice,
    /// Identifier of the message with purchase offer which was rejected or expired; may be 0 or an identifier of a deleted message
    pub offer_message_id: i64,
    /// True, if the offer has expired; otherwise, the offer was explicitly rejected
    pub was_expired: bool,
}

/// Paid messages were refunded
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaidMessagesRefunded {
    /// The number of refunded messages
    pub message_count: i32,
    /// The number of refunded Telegram Stars
    pub star_count: i64,
}

/// A price for paid messages was changed in the supergroup chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaidMessagePriceChanged {
    /// The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    pub paid_message_star_count: i64,
}

/// A price for direct messages was changed in the channel chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageDirectMessagePriceChanged {
    /// True, if direct messages group was enabled for the channel; false otherwise
    pub is_enabled: bool,
    /// The new number of Telegram Stars that must be paid by non-administrator users of the channel chat for each message sent to the direct messages group;
    /// 0 if the direct messages group was disabled or the messages are free
    pub paid_message_star_count: i64,
}

/// Some tasks from a checklist were marked as done or not done
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChecklistTasksDone {
    /// Identifier of the message with the checklist; may be 0 or an identifier of a deleted message
    pub checklist_message_id: i64,
    /// Identifiers of tasks that were marked as done
    pub marked_as_done_task_ids: Vec<i32>,
    /// Identifiers of tasks that were marked as not done
    pub marked_as_not_done_task_ids: Vec<i32>,
}

/// Some tasks were added to a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChecklistTasksAdded {
    /// Identifier of the message with the checklist; may be 0 or an identifier of a deleted message
    pub checklist_message_id: i64,
    /// List of tasks added to the checklist
    pub tasks: Vec<crate::types::ChecklistTask>,
}

/// Approval of suggested post has failed, because the user who proposed the post didn't have enough funds
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostApprovalFailed {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// Price of the suggested post
    pub price: crate::enums::SuggestedPostPrice,
}

/// A suggested post was approved
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostApproved {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// Price of the suggested post; may be null if the post is non-paid
    pub price: Option<crate::enums::SuggestedPostPrice>,
    /// Point in time (Unix timestamp) when the post is expected to be published
    pub send_date: i32,
}

/// A suggested post was declined
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostDeclined {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// Comment added by administrator of the channel when the post was declined
    pub comment: String,
}

/// A suggested post was published for getOption("suggested_post_lifetime_min") seconds and payment for the post was received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostPaid {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// The amount of received Telegram Stars
    pub star_amount: crate::types::StarAmount,
    /// The amount of received TON Grams; in the smallest units of the cryptocurrency
    pub gram_amount: i64,
}

/// A suggested post was refunded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageSuggestedPostRefunded {
    /// Identifier of the message with the suggested post; may be 0 or an identifier of a deleted message
    pub suggested_post_message_id: i64,
    /// Reason of the refund
    pub reason: crate::enums::SuggestedPostRefundReason,
}

/// A contact has registered with Telegram
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageContactRegistered {
}

/// The current user shared users who were requested by the bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageUsersShared {
    /// The shared users
    pub users: Vec<crate::types::SharedUser>,
    /// Identifier of the keyboard button with the request
    pub button_id: i32,
}

/// The current user shared a chat, which was requested by the bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatShared {
    /// The shared chat
    pub chat: crate::types::SharedChat,
    /// Identifier of the keyboard button with the request
    pub button_id: i32,
}

/// Data from a Web App has been sent to a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageWebAppDataSent {
    /// Text of the keyboardButtonTypeWebApp button, which opened the Web App
    pub button_text: String,
}

/// Data from a Web App has been received; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageWebAppDataReceived {
    /// Text of the keyboardButtonTypeWebApp button, which opened the Web App
    pub button_text: String,
    /// The data
    pub data: String,
}

/// A user in the chat came within proximity alert range
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageProximityAlertTriggered {
    /// The identifier of a user or chat that triggered the proximity alert
    pub traveler_id: crate::enums::MessageSender,
    /// The identifier of a user or chat that subscribed for the proximity alert
    pub watcher_id: crate::enums::MessageSender,
    /// The distance between the users
    pub distance: i32,
}

/// A message content that is not supported in the current TDLib version
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageUnsupported {
}

/// A mention of a user, a supergroup, or a channel by their username
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeMention {
}

/// A hashtag text, beginning with "#" and optionally containing a chat username at the end
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeHashtag {
}

/// A cashtag text, beginning with "$", consisting of capital English letters (e.g., "$USD"), and optionally containing a chat username at the end
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeCashtag {
}

/// An HTTP URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeUrl {
}

/// An email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeEmailAddress {
}

/// A phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypePhoneNumber {
}

/// A bank card number. The getBankCardInfo method can be used to get information about the bank card
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeBankCardNumber {
}

/// A bold text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeBold {
}

/// An italic text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeItalic {
}

/// An underlined text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeUnderline {
}

/// A strikethrough text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeStrikethrough {
}

/// A spoiler text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeSpoiler {
}

/// Text that must be formatted as if inside a code HTML tag
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeCode {
}

/// Text that must be formatted as if inside a pre HTML tag
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypePre {
}

/// Text that must be formatted as if inside pre, and code HTML tags
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypePreCode {
    /// Programming language of the code; as defined by the sender
    pub language: String,
}

/// Text that must be formatted as if inside a blockquote HTML tag; not supported in secret chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeBlockQuote {
}

/// Text that must be formatted as if inside a blockquote HTML tag and collapsed by default to 3 lines with the ability to show full text; not supported in secret chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeExpandableBlockQuote {
}

/// A text description shown instead of a raw URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeTextUrl {
    /// HTTP or tg: URL to be opened when the link is clicked
    pub url: String,
}

/// A text shows instead of a raw mention of the user (e.g., when the user has no username)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeMentionName {
    /// Identifier of the mentioned user
    pub user_id: i64,
}

/// A media timestamp
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeMediaTimestamp {
    /// Timestamp from which a video/audio/video note/voice note/story playing must start, in seconds. The media can be in the content or the link preview of the current message, or in the same places in the replied message
    pub media_timestamp: i32,
}

/// A date and time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeDateTime {
    /// Point in time (Unix timestamp) representing the date and time
    pub unix_time: i32,
    /// Date and time formatting type; may be null if none and the original text must not be changed
    pub formatting_type: Option<crate::enums::DateTimeFormattingType>,
}

/// The message will be sent at the specified date
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSchedulingStateSendAtDate {
    /// Point in time (Unix timestamp) when the message will be sent. The date must be within 367 days in the future
    pub send_date: i32,
    /// Period after which the message will be sent again; in seconds; 0 if never; for Telegram Premium users only; may be non-zero only in sendMessage and forwardMessages with one message requests;
    /// must be one of 0, 86400, 7 * 86400, 14 * 86400, 30 * 86400, 91 * 86400, 182 * 86400, 365 * 86400, or additionally 60, or 300 in the Test DC
    pub repeat_period: i32,
}

/// The message will be sent when the other user is online. Applicable to private chats only and when the exact online status of the other user is known
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSchedulingStateSendWhenOnline {
}

/// The message will be self-destructed in the specified time after its content was opened
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSelfDestructTypeTimer {
    /// The message's self-destruct time, in seconds; must be between 0 and 60 in private chats
    pub self_destruct_time: i32,
}

/// The message can be opened only once and will be self-destructed once closed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSelfDestructTypeImmediately {
}

/// Options to be used when a message is sent
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSendOptions {
    /// Information about the suggested post; pass null if none. For messages to channel direct messages chat only. Applicable only to sendMessage and addOffer
    pub suggested_post_info: Option<crate::types::InputSuggestedPostInfo>,
    /// Pass true to disable notification for the message
    pub disable_notification: bool,
    /// Pass true if the message is sent from the background
    pub from_background: bool,
    /// Pass true if the content of the message must be protected from forwarding and saving; for bots only
    pub protect_content: bool,
    /// Pass true to allow the message to ignore regular broadcast limits for a small fee; for bots only
    pub allow_paid_broadcast: bool,
    /// The number of Telegram Stars the user agreed to pay to send the messages
    pub paid_message_star_count: i64,
    /// Pass true if the user explicitly chosen a sticker or a custom emoji from an installed sticker set; applicable only to sendMessage and sendMessageAlbum
    pub update_order_of_installed_sticker_sets: bool,
    /// Message scheduling state; pass null to send message immediately. Messages sent to a secret chat, to a chat with paid messages, to a channel direct messages chat,
    /// live location messages and self-destructing messages can't be scheduled
    pub scheduling_state: Option<crate::enums::MessageSchedulingState>,
    /// Identifier of the effect to apply to the message; pass 0 if none; applicable only to sendMessage, sendMessageAlbum in private chats and forwardMessages with one message to private chats
    #[serde_as(as = "DisplayFromStr")]
    pub effect_id: i64,
    /// Non-persistent identifier, which will be returned back in messageSendingStatePending object and can be used to match sent messages and corresponding updateNewMessage updates
    pub sending_id: i32,
    /// Pass true to get a fake message instead of actually sending them
    pub only_preview: bool,
}

/// Options to be used when a message content is copied without reference to the original sender. Service messages, messages with messageInvoice, messagePaidMedia, messageGiveaway, or messageGiveawayWinners content can't be copied
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageCopyOptions {
    /// True, if content of the message needs to be copied without reference to the original sender. Always true if the message is forwarded to a secret chat or is local.
    /// Use messageProperties.can_be_copied and messageProperties.can_be_copied_to_secret_chat to check whether the message is suitable
    pub send_copy: bool,
    /// True, if media caption of the message copy needs to be replaced. Ignored if send_copy is false
    pub replace_caption: bool,
    /// New message caption; pass null to copy message without caption. Ignored if replace_caption is false
    pub new_caption: Option<crate::types::FormattedText>,
    /// True, if new caption must be shown above the media; otherwise, new caption must be shown below the media; not supported in secret chats. Ignored if replace_caption is false
    pub new_show_caption_above_media: bool,
}

/// A text message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageText {
    /// Formatted text to be sent; 0-getOption("message_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, BlockQuote, ExpandableBlockQuote,
    /// Code, Pre, PreCode, TextUrl, MentionName, and DateTime entities are allowed to be specified manually
    pub text: crate::types::FormattedText,
    /// Options to be used for generation of a link preview; may be null if none; pass null to use default link preview options
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
    /// Pass true to delete message draft in the chat
    pub clear_draft: bool,
}

/// A rich message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageRichMessage {
    /// The rich message to send
    pub message: crate::types::InputRichMessage,
    /// Pass true to delete message draft in the chat
    pub clear_draft: bool,
}

/// A message with paid media; can be used only in channel chats with supergroupFullInfo.has_paid_media_allowed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessagePaidMedia {
    /// The number of Telegram Stars that must be paid to see the media; 1-getOption("paid_media_message_star_count_max")
    pub star_count: i64,
    /// The content of the paid media
    pub paid_media: Vec<crate::types::InputPaidMedia>,
    /// Message caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
    /// True, if the caption must be shown above the media; otherwise, the caption must be shown below the media; not supported in secret chats
    pub show_caption_above_media: bool,
    /// Bot-provided data for the paid media; bots only
    pub payload: String,
}

/// A voice note message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageVoiceNote {
    /// Voice note to be sent
    pub voice_note: crate::types::InputVoiceNote,
    /// Voice note caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
    /// Voice note self-destruct type; may be null if none; pass null if none; private chats only
    pub self_destruct_type: Option<crate::enums::MessageSelfDestructType>,
}

/// A message with a live location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageLiveLocation {
    /// Initial state of the live location to be sent. Live period must be equal to 0x7FFFFFFF for permanent live locations, or between 60 and 86400
    pub location: crate::types::LiveLocation,
}

/// A message with a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageLocation {
    /// Location to be sent
    pub location: crate::types::Location,
}

/// A message with information about a venue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageVenue {
    /// Venue to send
    pub venue: crate::types::Venue,
}

/// A message containing a user contact
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageContact {
    /// Contact to send
    pub contact: crate::types::Contact,
}

/// A message with a game; not supported for channels or secret chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageGame {
    /// User identifier of the bot that owns the game
    pub bot_user_id: i64,
    /// Short name of the game
    pub game_short_name: String,
}

/// A message with a checklist. Checklists can't be sent to secret chats, channel chats and channel direct messages chats; for Telegram Premium users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageChecklist {
    /// The checklist to send
    pub checklist: crate::types::InputChecklist,
}

/// A forwarded message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageForwarded {
    /// Identifier for the chat this forwarded message came from
    pub from_chat_id: i64,
    /// Identifier of the message to forward. A message can be forwarded only if messageProperties.can_be_forwarded
    pub message_id: i64,
    /// Pass true if a game message is being shared from a launched game; applies only to game messages
    pub in_game_share: bool,
    /// Pass true to replace video start timestamp in the forwarded message
    pub replace_video_start_timestamp: bool,
    /// The new video start timestamp; ignored if replace_video_start_timestamp == false
    pub new_video_start_timestamp: i32,
    /// Options to be used to copy content of the message without reference to the original sender; pass null to forward the message as usual
    pub copy_options: Option<crate::types::MessageCopyOptions>,
}

/// Contains properties of a message and describes actions that can be done with the message right now
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageProperties {
    /// True, if an offer can be added to the message using addOffer
    pub can_add_offer: bool,
    /// True, if tasks can be added to the message's checklist using addChecklistTasks if the current user has Telegram Premium subscription
    pub can_add_tasks: bool,
    /// True, if the message is a suggested post that can be approved by the user using approveSuggestedPost
    pub can_be_approved: bool,
    /// True, if content of the message can be copied using inputMessageForwarded or forwardMessages with copy options
    pub can_be_copied: bool,
    /// True, if content of the message can be copied to a secret chat using inputMessageForwarded or forwardMessages with copy options
    pub can_be_copied_to_secret_chat: bool,
    /// True, if the message is a suggested post that can be declined by the user using declineSuggestedPost
    pub can_be_declined: bool,
    /// True, if the message can be deleted only for the current user while other users will continue to see it using the method deleteMessages with revoke == false
    pub can_be_deleted_only_for_self: bool,
    /// True, if the message can be deleted for all users using the method deleteMessages with revoke == true
    pub can_be_deleted_for_all_users: bool,
    /// True, if the message can be edited using the methods editMessageText, editMessageCaption, or editMessageReplyMarkup.
    /// For live location, poll, and checklist messages this fields shows whether editMessageLiveLocation, stopPoll, or editMessageChecklist respectively can be used with this message
    pub can_be_edited: bool,
    /// True, if the message can be forwarded using inputMessageForwarded or forwardMessages without copy options
    pub can_be_forwarded: bool,
    /// True, if the message can be paid using inputInvoiceMessage
    pub can_be_paid: bool,
    /// True, if the message can be pinned or unpinned in the chat using pinChatMessage or unpinChatMessage
    pub can_be_pinned: bool,
    /// True, if the message can be replied in the same chat and forum topic using inputMessageReplyToMessage. Ephemeral messages can be replied only by other ephemeral messages
    pub can_be_replied: bool,
    /// True, if the message can be replied in another chat or forum topic using inputMessageReplyToExternalMessage
    pub can_be_replied_in_another_chat: bool,
    /// True, if content of the message can be saved locally
    pub can_be_saved: bool,
    /// True, if the message can be shared in a story using inputStoryAreaTypeMessage
    pub can_be_shared_in_story: bool,
    /// True, if the user can delete reactions of other users in the message using the method deleteMessageReactionsFromSender
    pub can_delete_reactions: bool,
    /// True, if the message can be edited using the method editMessageMedia
    pub can_edit_media: bool,
    /// True, if scheduling state of the message can be edited
    pub can_edit_scheduling_state: bool,
    /// True, if another price or post send time can be suggested using addOffer
    pub can_edit_suggested_post_info: bool,
    /// True, if author of the message sent on behalf of a chat can be received through getMessageAuthor
    pub can_get_author: bool,
    /// True, if code for message embedding can be received using getMessageEmbeddingCode
    pub can_get_embedding_code: bool,
    /// True, if a link can be generated for the message using getMessageLink
    pub can_get_link: bool,
    /// True, if media timestamp links can be generated for media timestamp entities in the message text, caption or link preview description using getMessageLink
    pub can_get_media_timestamp_links: bool,
    /// True, if information about the message thread is available through getMessageThread and getMessageThreadHistory
    pub can_get_message_thread: bool,
    /// True, if the message is a poll and vote statistics are available through getPollVoteStatistics
    pub can_get_poll_vote_statistics: bool,
    /// True, if read date of the message can be received through getMessageReadDate
    pub can_get_read_date: bool,
    /// True, if message statistics are available through getMessageStatistics and message forwards can be received using getMessagePublicForwards
    pub can_get_statistics: bool,
    /// True, if advertisements for video of the message can be received through getVideoMessageAdvertisements
    pub can_get_video_advertisements: bool,
    /// True, if chat members already viewed the message can be received through getMessageViewers
    pub can_get_viewers: bool,
    /// True, if tasks can be marked as done or not done in the message's checklist using markChecklistTasksAsDone if the current user has Telegram Premium subscription
    pub can_mark_tasks_as_done: bool,
    /// True, if speech can be recognized for the message through recognizeSpeech
    pub can_recognize_speech: bool,
    /// True, if the message can be reported using reportChat
    pub can_report_chat: bool,
    /// True, if reactions on the message can be reported through reportMessageReactions
    pub can_report_reactions: bool,
    /// True, if the message can be reported using reportSupergroupSpam
    pub can_report_supergroup_spam: bool,
    /// True, if fact check for the message can be changed through setMessageFactCheck
    pub can_set_fact_check: bool,
    /// True, if content of the message can't be saved locally, because it is protected by the current user; if true, then can_be_saved is false
    pub has_protected_content_by_current_user: bool,
    /// True, if content of the message can't be saved locally, because it is protected by the other user; if true, then can_be_saved is false
    pub has_protected_content_by_other_user: bool,
    /// True, if message statistics must be available from context menu of the message
    pub need_show_statistics: bool,
}

/// Returns all found messages, no filter is applied
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterEmpty {
}

/// Returns only voice note messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterVoiceNote {
}

/// Returns only messages containing URLs
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterUrl {
}

/// Returns only messages with mentions of the current user, or messages that are replies to their messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterMention {
}

/// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query or by the sending user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterUnreadMention {
}

/// Returns only messages with unread reactions for the current user. When using this filter the results can't be additionally filtered by a query or by the sending user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterUnreadReaction {
}

/// Returns only failed to send messages. This filter can be used only if the message database is used
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterFailedToSend {
}

/// Returns only pinned messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesFilterPinned {
}

/// Returns only messages in private chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesChatTypeFilterPrivate {
}

/// Returns only messages in basic group and supergroup chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesChatTypeFilterGroup {
}

/// Returns only messages in channel chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesChatTypeFilterChannel {
}

/// Returns only messages in the specified community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchMessagesChatTypeFilterCommunity {
    /// Identifier of the community to search in
    pub community_id: i64,
}

/// Describes a message that can be used for quick reply
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct QuickReplyMessage {
    /// Unique message identifier among all quick replies
    pub id: i64,
    /// The sending state of the message; may be null if the message isn't being sent and didn't fail to be sent
    pub sending_state: Option<crate::enums::MessageSendingState>,
    /// True, if the message can be edited
    pub can_be_edited: bool,
    /// The identifier of the quick reply message to which the message replies; 0 if none
    pub reply_to_message_id: i64,
    /// If non-zero, the user identifier of the bot through which this message was sent
    pub via_bot_user_id: i64,
    /// Unique identifier of an album this message belongs to; 0 if none. Only audios, documents, photos and videos can be grouped together in albums
    #[serde_as(as = "DisplayFromStr")]
    pub media_album_id: i64,
    /// Content of the message
    pub content: crate::enums::MessageContent,
    /// Inline keyboard reply markup for the message; may be null if none
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
}

/// Contains a list of quick reply messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct QuickReplyMessages {
    /// List of quick reply messages; messages may be null
    pub messages: Vec<Option<crate::types::QuickReplyMessage>>,
}

/// Describes a set up welcome message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WelcomeMessage {
    /// Welcome message identifier; unique for the chat to which the welcome message belongs
    pub id: i32,
    /// Content of the welcome message
    pub content: crate::enums::MessageContent,
}

/// Contains a public forward as a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PublicForwardMessage {
    /// Information about the message
    pub message: crate::types::Message,
}

/// Represents a reaction applied to a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddedReaction {
    /// Type of the reaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::ReactionType,
    /// Identifier of the chat member, applied the reaction
    pub sender_id: crate::enums::MessageSender,
    /// True, if the reaction was added by the current user
    pub is_outgoing: bool,
    /// Point in time (Unix timestamp) when the reaction was added
    pub date: i32,
}

/// Represents a list of reactions added to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AddedReactions {
    /// The total number of found reactions
    pub total_count: i32,
    /// The list of added reactions
    pub reactions: Vec<crate::types::AddedReaction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Represents an available reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AvailableReaction {
    /// Type of the reaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::ReactionType,
    /// True, if Telegram Premium is needed to send the reaction
    pub needs_premium: bool,
}

/// Represents a list of reactions that can be added to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AvailableReactions {
    /// List of reactions to be shown at the top
    pub top_reactions: Vec<crate::types::AvailableReaction>,
    /// List of recently used reactions
    pub recent_reactions: Vec<crate::types::AvailableReaction>,
    /// List of popular reactions
    pub popular_reactions: Vec<crate::types::AvailableReaction>,
    /// True, if any custom emoji reaction can be added by Telegram Premium subscribers
    pub allow_custom_emoji: bool,
    /// True, if the reactions will be tags and the message can be found by them
    pub are_tags: bool,
    /// The reason why the current user can't add reactions to the message, despite some other users can; may be null if none
    pub unavailability_reason: Option<crate::enums::ReactionUnavailabilityReason>,
}

/// The user is an anonymous administrator in the supergroup, but isn't a creator of it, so they can't vote on behalf of the supergroup
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionUnavailabilityReasonAnonymousAdministrator {
}

/// The user isn't a member of the supergroup and can't send messages and reactions there without joining
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionUnavailabilityReasonGuest {
}

/// The user is restricted in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionUnavailabilityReasonRestricted {
}

/// The speech recognition successfully finished
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SpeechRecognitionResultText {
    /// Recognized text
    pub text: String,
}

/// Contains identifier of a sent guest message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineMessageId {
    /// Unique identifier for the message
    pub id: String,
}

/// Represents an inline message that can be sent via the bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PreparedInlineMessageId {
    /// Unique identifier for the message
    pub id: String,
    /// Point in time (Unix timestamp) when the message can't be used anymore
    pub expiration_date: i32,
}

/// Represents a ready to send inline message. Use sendInlineQueryResultMessage to send the message
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the inline query to pass to sendInlineQueryResultMessage
    #[serde_as(as = "DisplayFromStr")]
    pub inline_query_id: i64,
    /// Resulted inline message of the query
    pub result: crate::enums::InlineQueryResult,
    /// Types of the chats to which the message can be sent
    pub chat_types: crate::types::TargetChatTypes,
}

/// A message was edited
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessageEdited {
    /// The original message before the edit
    pub old_message: crate::types::Message,
    /// The message after it was edited
    pub new_message: crate::types::Message,
}

/// A message was deleted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessageDeleted {
    /// Deleted message
    pub message: crate::types::Message,
    /// True, if the message deletion can be reported via reportSupergroupAntiSpamFalsePositive
    pub can_report_anti_spam_false_positive: bool,
}

/// A message was pinned
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessagePinned {
    /// Pinned message
    pub message: crate::types::Message,
}

/// A message was unpinned
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessageUnpinned {
    /// Unpinned message
    pub message: crate::types::Message,
}

/// The chat available reactions were changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventAvailableReactionsChanged {
    /// Previous chat available reactions
    pub old_available_reactions: crate::enums::ChatAvailableReactions,
    /// New chat available reactions
    pub new_available_reactions: crate::enums::ChatAvailableReactions,
}

/// The message auto-delete timer was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessageAutoDeleteTimeChanged {
    /// Previous value of message_auto_delete_time
    pub old_message_auto_delete_time: i32,
    /// New value of message_auto_delete_time
    pub new_message_auto_delete_time: i32,
}

/// The sign_messages setting of a channel was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventSignMessagesToggled {
    /// New value of sign_messages
    pub sign_messages: bool,
}

/// The show_message_sender setting of a channel was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventShowMessageSenderToggled {
    /// New value of show_message_sender
    pub show_message_sender: bool,
}

/// A general message with hidden content
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentHidden {
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A message with a user contact
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentContact {
    /// Contact's name
    pub name: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A contact has registered with Telegram
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentContactRegistered {
    /// True, if the user joined Telegram as a Telegram Premium account
    pub as_premium_account: bool,
}

/// A message with a game
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentGame {
    /// Game title, empty for pinned game message
    pub title: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A new high score was achieved in a game
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentGameScore {
    /// Game title, empty for pinned message
    pub title: String,
    /// New score, 0 for pinned message
    pub score: i32,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A message with a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentLocation {
    /// True, if the location is live
    pub is_live: bool,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A message with paid media
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentPaidMedia {
    /// Number of Telegram Stars needed to buy access to the media in the message; 0 for pinned message
    pub star_count: i64,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A message with a gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentGift {
    /// Number of Telegram Stars that sender paid for the gift
    pub star_count: i64,
    /// True, if the message is about prepaid upgrade of the gift by another user instead of actual receiving of a new gift
    pub is_prepaid_upgrade: bool,
}

/// A message with an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentUpgradedGift {
    /// True, if the gift was obtained by upgrading of a previously received gift; otherwise, if is_prepaid_upgrade == false, then this is a transferred or resold gift
    pub is_upgrade: bool,
    /// True, if the message is about completion of prepaid upgrade of the gift instead of actual receiving of a new gift
    pub is_prepaid_upgrade: bool,
}

/// A screenshot of a message in the chat has been taken
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentScreenshotTaken {
}

/// A text message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentText {
    /// Message text
    pub text: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A message with a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChecklist {
    /// Checklist title
    pub title: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A voice note message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentVoiceNote {
    /// Message content; may be null
    pub voice_note: Option<crate::types::VoiceNote>,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// A newly created basic group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentBasicGroupChatCreate {
}

/// New chat members were invited to a group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatAddMembers {
    /// Name of the added member
    pub member_name: String,
    /// True, if the current user was added to the group
    pub is_current_user: bool,
    /// True, if the user has returned to the group themselves
    pub is_returned: bool,
}

/// A chat title was edited
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatChangeTitle {
    /// New chat title
    pub title: String,
}

/// A chat background was edited
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatSetBackground {
    /// True, if the set background is the same as the background of the current user
    pub is_same: bool,
}

/// A chat theme was edited
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatSetTheme {
    /// If non-empty, human-readable name of the new theme. Otherwise, the chat theme was reset to the default one
    pub name: String,
}

/// A chat member was deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatDeleteMember {
    /// Name of the deleted member
    pub member_name: String,
    /// True, if the current user was deleted from the group
    pub is_current_user: bool,
    /// True, if the user has left the group themselves
    pub is_left: bool,
}

/// A new member joined the chat via an invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatJoinByLink {
}

/// A new member was accepted to the chat by an administrator
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatJoinByRequest {
}

/// A birthdate was suggested to be set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentSuggestBirthdate {
}

/// A user in the chat came within proximity alert range from the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentProximityAlertTriggered {
    /// The distance to the user
    pub distance: i32,
}

/// Some tasks were added to a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChecklistTasksAdded {
    /// Number of added tasks
    pub task_count: i32,
}

/// Some tasks from a checklist were marked as done or not done
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChecklistTasksDone {
    /// Number of changed tasks
    pub task_count: i32,
}

/// A forwarded messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentMessageForwards {
    /// Number of forwarded messages
    pub total_count: i32,
}

/// A media album
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentMediaAlbum {
    /// Number of messages in the album
    pub total_count: i32,
    /// True, if the album has at least one photo
    pub has_photos: bool,
    /// True, if the album has at least one video file
    pub has_videos: bool,
    /// True, if the album has at least one audio file
    pub has_audios: bool,
    /// True, if the album has at least one document
    pub has_documents: bool,
}

/// A privacy setting for managing whether a link to the user's account is included in forwarded messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingShowLinkInForwardedMessages {
}

/// A privacy setting for managing whether the user can receive messages without additional payment
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingAllowUnpaidMessages {
}

/// The user can be messaged
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanSendMessageToUserResultOk {
}

/// The user can be messaged, but the messages are paid
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanSendMessageToUserResultUserHasPaidMessages {
    /// Number of Telegram Stars that must be paid by the current user for each sent message to the user
    pub outgoing_paid_message_star_count: i64,
}

/// The user can't be messaged, because they are deleted or unknown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanSendMessageToUserResultUserIsDeleted {
}

/// The user can't be messaged, because they restrict new chats with non-contacts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanSendMessageToUserResultUserRestrictsNewChats {
}

/// Contains default auto-delete timer setting for new chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAutoDeleteTime {
    /// Message auto-delete time, in seconds. If 0, then messages aren't deleted automatically
    pub time: i32,
}

/// The user must add additional text details to the report
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportChatResultTextRequired {
    /// Option identifier for the next reportChat request
    pub option_id: String,
    /// True, if the user can skip text adding
    pub is_optional: bool,
}

/// The user must choose messages to report and repeat the reportChat request with the chosen messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportChatResultMessagesRequired {
}

/// The link is a link to a channel direct messages chat by username of the channel. Call searchPublicChat with the given chat username to process the link.
/// If the chat is found and is channel, open the direct messages chat of the channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeDirectMessagesChat {
    /// Username of the channel
    pub channel_username: String,
}

/// The link is a link to a Telegram message or a forum topic. Call getMessageLinkInfo with the given URL to process the link,
/// and then open received forum topic or chat and show the message there
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeMessage {
    /// URL to be passed to getMessageLinkInfo
    pub url: String,
}

/// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeMessageDraft {
    /// Message draft text
    pub text: crate::types::FormattedText,
    /// True, if the first line of the text contains a link. If true, the input field needs to be focused and the text after the link must be selected
    pub contains_link: bool,
}

/// The link is a link to the Saved Messages chat. Call createPrivateChat with getOption("my_id") and open the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeSavedMessages {
}

/// The link is a link to a text composition style. Call searchTextCompositionStyle with the given style name to get information about the style.
/// If the style is found and the user wants to add it, then call addTextCompositionStyle
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeTextCompositionStyle {
    /// Name of the style
    pub style_name: String,
}

/// Contains an HTTPS link to a message in a supergroup or channel, or a forum topic
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageLink {
    /// The link
    pub link: String,
    /// True, if the link will work for non-members of the chat
    pub is_public: bool,
}

/// Contains information about a link to a message or a forum topic in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageLinkInfo {
    /// True, if the link is a public link for a message or a forum topic in a chat
    pub is_public: bool,
    /// If found, identifier of the chat to which the link points, 0 otherwise
    pub chat_id: i64,
    /// Identifier of the specific topic in which the message must be opened, or a topic to open if the message is missing; may be null if none
    pub topic_id: Option<crate::enums::MessageTopic>,
    /// If found, the linked message; may be null
    pub message: Option<crate::types::Message>,
    /// Timestamp from which the video/audio/video note/voice note/story playing must start, in seconds; 0 if not specified. The media can be in the message content or in its link preview
    pub media_timestamp: i32,
    /// Identifier of the checklist task that is linked; 0 if none
    pub checklist_task_id: i32,
    /// Identifier of the poll option that is linked; empty if none
    pub poll_option_id: String,
    /// True, if the whole media album to which the message belongs is linked
    pub for_album: bool,
}

/// Contains some text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Text {
    /// Text
    pub text: String,
}

/// The text uses Markdown-style formatting
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextParseModeMarkdown {
    /// Version of the parser: 0 or 1 - Telegram Bot API "Markdown" parse mode, 2 - Telegram Bot API "MarkdownV2" parse mode
    pub version: i32,
}

/// The text uses HTML-style formatting. The same as Telegram Bot API "HTML" parse mode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextParseModeHtml {
}

/// Describes a message sent in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsObjectTypeMessage {
    /// Message identifier
    pub message_id: i64,
}

/// Contains statistics about messages sent by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsMessageSenderInfo {
    /// User identifier
    pub user_id: i64,
    /// Number of sent messages
    pub sent_message_count: i32,
    /// Average number of characters in sent messages; 0 if unknown
    pub average_character_count: i32,
}

/// A detailed statistics about a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageStatistics {
    /// A graph containing number of message views and shares
    pub message_interaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of message reactions
    pub message_reaction_graph: crate::enums::StatisticalGraph,
}

/// A new message was received; can also be an outgoing message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewMessage {
    /// The new message
    pub message: crate::types::Message,
}

/// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully.
/// This update is sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageSendAcknowledged {
    /// The chat identifier of the sent message
    pub chat_id: i64,
    /// A temporary message identifier
    pub message_id: i64,
}

/// A message has been successfully sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageSendSucceeded {
    /// The sent message. Almost any field of the new message can be different from the corresponding field of the original message.
    /// For example, the field scheduling_state may change, making the message scheduled, or non-scheduled
    pub message: crate::types::Message,
    /// The previous temporary message identifier
    pub old_message_id: i64,
}

/// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageSendFailed {
    /// The failed to send message
    pub message: crate::types::Message,
    /// The previous temporary message identifier
    pub old_message_id: i64,
    /// The cause of the message sending failure
    pub error: crate::types::Error,
}

/// The message content has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageContent {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// New message content
    pub new_content: crate::enums::MessageContent,
}

/// The message ephemeral content has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageEphemeralContent {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// New ephemeral content of the message; may be null if none
    pub ephemeral_content: Option<crate::types::EphemeralMessageContent>,
}

/// A message was edited. Changes in the message content will come in a separate updateMessageContent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageEdited {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the message was edited
    pub edit_date: i32,
    /// New message reply markup; may be null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
}

/// The message pinned state was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageIsPinned {
    /// Chat identifier
    pub chat_id: i64,
    /// The message identifier
    pub message_id: i64,
    /// True, if the message is pinned
    pub is_pinned: bool,
}

/// The information about interactions with a message has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageInteractionInfo {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// New information about interactions with the message; may be null
    pub interaction_info: Option<crate::types::MessageInteractionInfo>,
}

/// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the self-destruct timer
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageContentOpened {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
}

/// A message with an unread mention was read
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageMentionRead {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The new number of unread mention messages left in the chat
    pub unread_mention_count: i32,
}

/// The list of unread reactions added to a message was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageUnreadReactions {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The new list of unread reactions
    pub unread_reactions: Vec<crate::types::UnreadReaction>,
    /// The new number of messages with unread reactions in the chat
    pub unread_reaction_count: i32,
}

/// A fact-check added to a message was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageFactCheck {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The new fact-check
    pub fact_check: crate::types::FactCheck,
}

/// Information about suggested post of a message was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageSuggestedPostInfo {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The new information about the suggested post
    pub suggested_post_info: crate::types::SuggestedPostInfo,
}

/// A message with a live location was viewed. When the update is received, the application is expected to update the live location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageLiveLocationViewed {
    /// Identifier of the chat with the live location message
    pub chat_id: i64,
    /// Identifier of the message with live location
    pub message_id: i64,
}

/// The last message of a chat was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatLastMessage {
    /// Chat identifier
    pub chat_id: i64,
    /// The new last message in the chat; may be null if the last message became unknown. While the last message is unknown, new messages can be added to the chat without corresponding updateNewMessage update
    pub last_message: Option<crate::types::Message>,
    /// The new chat positions in the chat lists
    pub positions: Vec<crate::types::ChatPosition>,
}

/// The chat available reactions were changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatAvailableReactions {
    /// Chat identifier
    pub chat_id: i64,
    /// The new reactions, available in the chat
    pub available_reactions: crate::enums::ChatAvailableReactions,
}

/// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatDraftMessage {
    /// Chat identifier
    pub chat_id: i64,
    /// The new draft message; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
    /// The new chat positions in the chat lists
    pub positions: Vec<crate::types::ChatPosition>,
}

/// The message sender that is selected to send messages in a chat has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatMessageSender {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of message_sender_id; may be null if the user can't change message sender
    pub message_sender_id: Option<crate::enums::MessageSender>,
}

/// The message auto-delete or self-destruct timer setting for a chat was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatMessageAutoDeleteTime {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of message_auto_delete_time
    pub message_auto_delete_time: i32,
}

/// The chat unread_reaction_count has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatUnreadReactionCount {
    /// Chat identifier
    pub chat_id: i64,
    /// The number of messages with unread reactions left in the chat
    pub unread_reaction_count: i32,
}

/// A chat's has_scheduled_messages field has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatHasScheduledMessages {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of has_scheduled_messages
    pub has_scheduled_messages: bool,
}

/// A chat's has_welcome_messages field has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatHasWelcomeMessages {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of has_welcome_messages
    pub has_welcome_messages: bool,
}

/// The list of quick reply shortcut messages has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateQuickReplyShortcutMessages {
    /// The identifier of the shortcut
    pub shortcut_id: i32,
    /// The new list of quick reply messages for the shortcut in order from the first to the last sent
    pub messages: Vec<crate::types::QuickReplyMessage>,
}

/// The list of welcome messages of a chat has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatWelcomeMessages {
    /// The identifier of the chat
    pub chat_id: i64,
    /// The new list of welcome messages of the chat in the order from the first to the last sent
    pub messages: Vec<crate::types::WelcomeMessage>,
}

/// Some messages were deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateDeleteMessages {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifiers of the deleted messages
    pub message_ids: Vec<i64>,
    /// True, if the messages are permanently deleted by a user (as opposed to just becoming inaccessible)
    pub is_permanent: bool,
    /// True, if the messages are deleted only from the cache and can possibly be retrieved again in the future
    pub from_cache: bool,
}

/// A new pending text or rich message was received in a chat with a bot. The message must be shown in the chat for at most getOption("pending_text_message_period") seconds,
/// replace any other pending message with the same draft_id with animation, and be deleted whenever any incoming message or a pending message with another draft_id is received in the message thread
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdatePendingMessage {
    /// Chat identifier
    pub chat_id: i64,
    /// The forum topic identifier in which the message will be sent; 0 if none
    pub forum_topic_id: i32,
    /// Unique identifier of the message draft within the message thread
    #[serde_as(as = "DisplayFromStr")]
    pub draft_id: i64,
    /// True, if a button that calls stopPendingMessage to stop further message generation must be shown
    pub can_stop: bool,
    /// True, if the pending message must not be automatically deleted when the user presses the Stop button
    pub keep_on_stop: bool,
    /// Content of the message; always of the type messageText or messageRichMessage
    pub content: crate::enums::MessageContent,
}

/// A message draft generation was stopped by the user
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateStopMessageDraft {
    /// Chat identifier
    pub chat_id: i64,
    /// The forum topic identifier of the message draft
    pub forum_topic_id: i32,
    /// Identifier of the message draft within the message thread
    #[serde_as(as = "DisplayFromStr")]
    pub draft_id: i64,
}

/// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUnreadMessageCount {
    /// The chat list with changed number of unread messages
    pub chat_list: crate::enums::ChatList,
    /// Total number of unread messages
    pub unread_count: i32,
    /// Total number of unread messages in unmuted chats
    pub unread_unmuted_count: i32,
}

/// A message was sent by an opened Web App, so the Web App needs to be closed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateWebAppMessageSent {
    /// Identifier of Web App launch
    #[serde_as(as = "DisplayFromStr")]
    pub web_app_launch_id: i64,
}

/// The list of available message effects has changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAvailableMessageEffects {
    /// The new list of available message effects from emoji reactions
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub reaction_effect_ids: Vec<i64>,
    /// The new list of available message effects from Premium stickers
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub sticker_effect_ids: Vec<i64>,
}

/// The type of default reaction has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateDefaultReactionType {
    /// The new type of the default reaction
    pub reaction_type: crate::enums::ReactionType,
}

/// The type of default paid reaction has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateDefaultPaidReactionType {
    /// The new type of the default paid reaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::PaidReactionType,
}

/// Tags used in Saved Messages or a Saved Messages topic have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedMessagesTags {
    /// Identifier of Saved Messages topic which tags were changed; 0 if tags for the whole chat has changed
    pub saved_messages_topic_id: i64,
    /// The new tags
    pub tags: crate::types::SavedMessagesTags,
}

/// The list of messages with active live location that need to be updated by the application has changed. The list is persistent across application restarts only if the message database is used
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateActiveLiveLocationMessages {
    /// The list of messages with active live locations
    pub messages: Vec<crate::types::Message>,
}

/// The styles supported for text composition have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateTextCompositionStyles {
    /// The new list of supported styles
    pub styles: Vec<crate::types::TextCompositionStyle>,
}

/// User changed its reactions on a message with public reactions; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageReaction {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// Identifier of the user or chat that changed reactions
    pub actor_id: crate::enums::MessageSender,
    /// Point in time (Unix timestamp) when the reactions were changed
    pub date: i32,
    /// Old list of chosen reactions
    pub old_reaction_types: Vec<crate::enums::ReactionType>,
    /// New list of chosen reactions
    pub new_reaction_types: Vec<crate::enums::ReactionType>,
}

/// Reactions added to a message with anonymous reactions have changed; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageReactions {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the reactions were changed
    pub date: i32,
    /// The list of reactions added to the message
    pub reactions: Vec<crate::types::MessageReaction>,
}


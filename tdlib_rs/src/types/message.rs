#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a message
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Message {
    /// Message identifier; unique for the chat to which the message belongs
    pub id: i64,
    /// Identifier of the sender of the message
    pub sender_id: crate::enums::MessageSender,
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
    /// True, if the message is a suggested channel post which was paid in Toncoins; a warning must be shown if the message is deleted in less than getOption("suggested_post_lifetime_min") seconds after sending
    pub is_paid_ton_suggested_post: bool,
    /// True, if the message contains an unread mention for the current user
    pub contains_unread_mention: bool,
    /// Point in time (Unix timestamp) when the message was sent; 0 for scheduled messages
    pub date: i32,
    /// Point in time (Unix timestamp) when the message was last edited; 0 for scheduled messages
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
    /// Reply markup for the message; may be null if none
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
}

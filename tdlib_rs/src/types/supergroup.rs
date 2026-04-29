#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup:
/// only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos.
/// Unlike supergroups, channels can have an unlimited number of subscribers
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Supergroup {
    /// Supergroup or channel identifier
    pub id: i64,
    /// Usernames of the supergroup or channel; may be null
    pub usernames: Option<crate::types::Usernames>,
    /// Point in time (Unix timestamp) when the current user joined, or the point in time when the supergroup or channel was created, in case the user is not a member
    pub date: i32,
    /// Status of the current user in the supergroup or channel
    pub status: crate::enums::ChatMemberStatus,
    /// Number of members in the supergroup or channel; 0 if unknown. Currently, it is guaranteed to be known only if the supergroup or channel was received through
    /// getChatSimilarChats, getChatsToPostStories, getCreatedPublicChats, getGroupsInCommon, getInactiveSupergroupChats, getRecommendedChats, getSuitableDiscussionChats,
    /// getUserPrivacySettingRules, getVideoChatAvailableParticipants, searchPublicChats, or in chatFolderInviteLinkInfo.missing_chat_ids, or in userFullInfo.personal_chat_id,
    /// or for chats with messages or stories from publicForwards and foundStories
    pub member_count: i32,
    /// Approximate boost level for the chat
    pub boost_level: i32,
    /// True, if automatic translation of messages is enabled in the channel
    pub has_automatic_translation: bool,
    /// True, if the channel has a discussion group, or the supergroup is the designated discussion group for a channel
    pub has_linked_chat: bool,
    /// True, if the supergroup is connected to a location, i.e. the supergroup is a location-based supergroup
    pub has_location: bool,
    /// True, if messages sent to the channel contains name of the sender. This field is only applicable to channels
    pub sign_messages: bool,
    /// True, if messages sent to the channel have information about the sender user. This field is only applicable to channels
    pub show_message_sender: bool,
    /// True, if users need to join the supergroup before they can send messages. May be false only for discussion supergroups and channel direct messages groups
    pub join_to_send_messages: bool,
    /// True, if all users directly joining the supergroup need to be approved by supergroup administrators. May be true only for non-broadcast supergroups with username, location, or a linked chat
    pub join_by_request: bool,
    /// True, if the slow mode is enabled in the supergroup
    pub is_slow_mode_enabled: bool,
    /// True, if the supergroup is a channel
    pub is_channel: bool,
    /// True, if the supergroup is a broadcast group, i.e. only administrators can send messages and there is no limit on the number of members
    pub is_broadcast_group: bool,
    /// True, if the supergroup is a forum with topics
    pub is_forum: bool,
    /// True, if the supergroup is a direct message group for a channel chat
    pub is_direct_messages_group: bool,
    /// True, if the supergroup is a direct messages group for a channel chat that is administered by the current user
    pub is_administered_direct_messages_group: bool,
    /// Information about verification status of the supergroup or channel; may be null if none
    pub verification_status: Option<crate::types::VerificationStatus>,
    /// True, if the channel has direct messages group
    pub has_direct_messages_group: bool,
    /// True, if the supergroup is a forum, which topics are shown in the same way as in channel direct messages groups
    pub has_forum_tabs: bool,
    /// Information about the restrictions that must be applied to the corresponding supergroup or channel chat; may be null if none
    pub restriction_info: Option<crate::types::RestrictionInfo>,
    /// Number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    pub paid_message_star_count: i64,
    /// State of active stories of the supergroup or channel; may be null if there are no active stories
    pub active_story_state: Option<crate::enums::ActiveStoryState>,
}

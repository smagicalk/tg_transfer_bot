//!
//! TDLib `chat` domain types.
//!
//! Types, enums, and functions for managing private chats, basic groups, supergroups, and channels.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a background set for a specific chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatBackground {
    /// The background
    pub background: crate::types::Background,
    /// Dimming of the background in dark themes, as a percentage; 0-100. Applied only to Wallpaper and Fill types of background
    pub dark_theme_dimming: i32,
}

/// Represents a location to which a chat is connected
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatLocation {
    /// The location
    pub location: crate::types::Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}

/// Describes actions that a user is allowed to take in a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatPermissions {
    /// True, if the user can send text messages, rich messages, contacts, giveaways, giveaway winners, invoices, locations, and venues
    pub can_send_basic_messages: bool,
    /// True, if the user can send music files
    pub can_send_audios: bool,
    /// True, if the user can send documents
    pub can_send_documents: bool,
    /// True, if the user can send photos
    pub can_send_photos: bool,
    /// True, if the user can send videos
    pub can_send_videos: bool,
    /// True, if the user can send video notes
    pub can_send_video_notes: bool,
    /// True, if the user can send voice notes
    pub can_send_voice_notes: bool,
    /// True, if the user can send polls and checklists
    pub can_send_polls: bool,
    /// True, if the user can send animations, games, stickers, and dice and use inline bots
    pub can_send_other_messages: bool,
    /// True, if the user may add a link preview to their messages
    pub can_add_link_previews: bool,
    /// True, if the user can react to messages
    pub can_react_to_messages: bool,
    /// True, if the user may change the tag of self
    pub can_edit_tag: bool,
    /// True, if the user can change the chat title, photo, and other settings
    pub can_change_info: bool,
    /// True, if the user can invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the user can pin messages
    pub can_pin_messages: bool,
    /// True, if the user can create topics
    pub can_create_topics: bool,
}

/// Describes rights of the administrator
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAdministratorRights {
    /// True, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report supergroup spam messages,
    /// ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other privilege; applicable to supergroups and channels only
    pub can_manage_chat: bool,
    /// True, if the administrator can change the chat title, photo, and other settings
    pub can_change_info: bool,
    /// True, if the administrator can create channel posts, approve suggested channel posts, or view channel statistics; applicable to channels only
    pub can_post_messages: bool,
    /// True, if the administrator can edit messages of other users and pin messages; applicable to channels only
    pub can_edit_messages: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can restrict, ban, or unban chat members or view supergroup statistics
    pub can_restrict_members: bool,
    /// True, if the administrator can pin messages; applicable to basic groups and supergroups only
    pub can_pin_messages: bool,
    /// True, if the administrator can create, rename, close, reopen, hide, and unhide forum topics; applicable to forum supergroups only
    pub can_manage_topics: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that were directly or indirectly promoted by them; applicable to supergroups and channels only
    pub can_promote_members: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can create new chat stories, or edit and delete posted stories; applicable to supergroups and channels only
    pub can_post_stories: bool,
    /// True, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access story archive; applicable to supergroups and channels only
    pub can_edit_stories: bool,
    /// True, if the administrator can delete stories posted by other users; applicable to supergroups and channels only
    pub can_delete_stories: bool,
    /// True, if the administrator can answer to channel direct messages; applicable to channels only
    pub can_manage_direct_messages: bool,
    /// True, if the administrator can change tags of other users; applicable to basic groups and supergroups only
    pub can_manage_tags: bool,
    /// True, if the administrator can manage and send welcome messages
    pub can_send_welcome_messages: bool,
    /// True, if the administrator isn't shown in the chat member list and sends messages anonymously; applicable to supergroups only
    pub is_anonymous: bool,
}

/// The transaction is a purchase of paid media from a channel by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelPaidMediaPurchase {
    /// Identifier of the channel chat that sent the paid media
    pub chat_id: i64,
    /// Identifier of the corresponding message with paid media; may be 0 or an identifier of a deleted message
    pub message_id: i64,
    /// The bought media if the transaction wasn't refunded
    pub media: Vec<crate::enums::PaidMedia>,
}

/// The transaction is a sale of paid media by the channel chat; relevant for channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelPaidMediaSale {
    /// Identifier of the user who bought the media
    pub user_id: i64,
    /// Identifier of the corresponding message with paid media; may be 0 or an identifier of a deleted message
    pub message_id: i64,
    /// The bought media
    pub media: Vec<crate::enums::PaidMedia>,
}

/// The transaction is a purchase of a subscription to a channel chat by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelSubscriptionPurchase {
    /// Identifier of the channel chat that created the subscription
    pub chat_id: i64,
    /// The number of seconds between consecutive Telegram Star debitings
    pub subscription_period: i32,
}

/// The transaction is a sale of a subscription by the channel chat; relevant for channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelSubscriptionSale {
    /// Identifier of the user who bought the subscription
    pub user_id: i64,
    /// The number of seconds between consecutive Telegram Star debitings
    pub subscription_period: i32,
}

/// Describes a chat in a community
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CommunityChat {
    /// Identifier of the chat in the community
    pub chat_id: i64,
    /// True, if message history of the chat can be viewed
    pub can_view_history: bool,
    /// True, if the chat is hidden in the list of community chats; for community administrators only
    pub is_hidden: bool,
}

/// Contains information about a chat administrator
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAdministrator {
    /// User identifier of the administrator
    pub user_id: i64,
    /// Custom title of the administrator
    pub custom_title: String,
    /// True, if the user is the owner of the chat
    pub is_owner: bool,
    /// True, if the current user can edit the administrator privileges for the administrator
    pub can_be_edited: bool,
}

/// Represents a list of chat administrators
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAdministrators {
    /// A list of chat administrators
    pub administrators: Vec<crate::types::ChatAdministrator>,
}

/// The user is the owner of the chat and has all the administrator privileges
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusCreator {
    /// True, if the creator isn't shown in the chat member list and sends messages anonymously; applicable to supergroups only
    pub is_anonymous: bool,
    /// True, if the user is a member of the chat
    pub is_member: bool,
}

/// The user is a member of the chat and has some additional privileges. In basic groups, administrators have all applicable rights.
/// In supergroups and channels, any subset of the rights can be chosen for an administrator
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusAdministrator {
    /// True, if the current user can edit the administrator privileges for the called user
    pub can_be_edited: bool,
    /// Rights of the administrator
    pub rights: crate::types::ChatAdministratorRights,
}

/// The user is a member of the chat, without any additional privileges or restrictions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusMember {
    /// Point in time (Unix timestamp) when the user will be removed from the chat because of the expired subscription; 0 if never. Ignored in setChatMemberStatus
    pub member_until_date: i32,
}

/// The user is under certain restrictions in the chat. Not supported in basic groups and channels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusRestricted {
    /// True, if the user is a member of the chat
    pub is_member: bool,
    /// Point in time (Unix timestamp) when restrictions will be lifted from the user; 0 if never. If the user is restricted for more than 366 days or for less than 30 seconds from the current time, the user is considered to be restricted forever
    pub restricted_until_date: i32,
    /// User permissions in the chat
    pub permissions: crate::types::ChatPermissions,
}

/// The user or the chat is not a chat member
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusLeft {
}

/// The user or the chat was banned (and hence is not a member of the chat). Implies the user can't return to the chat, view messages, or be used as a participant identifier to join a video chat of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusBanned {
    /// Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever. Always 0 in basic groups
    pub banned_until_date: i32,
}

/// Describes a user or a chat as a member of another chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatMember {
    /// Identifier of the chat member. Currently, other chats can be only Left or Banned. Only supergroups and channels can have other chats as Left or Banned members and these chats must be supergroups or channels
    pub member_id: crate::enums::MessageSender,
    /// Tag of the chat member or its custom title if the member is an administrator of the chat; 0-16 characters without emoji; applicable to basic groups and supergroups only
    pub tag: String,
    /// Identifier of a user who invited/promoted/banned this member in the chat; 0 if unknown
    pub inviter_user_id: i64,
    /// Point in time (Unix timestamp) when the user joined/was promoted/was banned in the chat
    pub joined_chat_date: i32,
    /// Status of the member in the chat
    pub status: crate::enums::ChatMemberStatus,
}

/// Contains a list of chat members
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembers {
    /// Approximate total number of chat members found
    pub total_count: i32,
    /// A list of chat members
    pub members: Vec<crate::types::ChatMember>,
}

/// Returns contacts of the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembersFilterContacts {
}

/// Returns the owner and administrators
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembersFilterAdministrators {
}

/// Returns all chat members, including restricted chat members
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembersFilterMembers {
}

/// Returns users who can be mentioned in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembersFilterMention {
    /// Identifier of the topic in which the users will be mentioned; pass null if none
    pub topic_id: Option<crate::enums::MessageTopic>,
}

/// Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembersFilterRestricted {
}

/// Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembersFilterBanned {
}

/// Returns recently active users in reverse chronological order
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterRecent {
}

/// Returns contacts of the current user who are members of the supergroup or channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterContacts {
    /// Query to search for
    pub query: String,
}

/// Returns the owner and administrators
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterAdministrators {
}

/// Used to search for supergroup or channel members via a (string) query
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterSearch {
    /// Query to search for
    pub query: String,
}

/// Returns restricted supergroup members; can be used only by administrators
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterRestricted {
    /// Query to search for
    pub query: String,
}

/// Returns users banned from the supergroup or channel; can be used only by administrators
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterBanned {
    /// Query to search for
    pub query: String,
}

/// Returns users who can be mentioned in the supergroup
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterMention {
    /// Query to search for
    pub query: String,
    /// Identifier of the topic in which the users will be mentioned; pass null if none
    pub topic_id: Option<crate::enums::MessageTopic>,
}

/// The chat was joined successfully
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinResultSuccess {
    /// Identifier of the chat
    pub chat_id: i64,
}

/// The join request was sent and have to be approved by administrators of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinResultRequestSent {
}

/// The join was declined by the guard bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinResultDeclined {
}

/// The request was approved
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequestResultApproved {
}

/// The request was declined
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequestResultDeclined {
}

/// The request was postponed without a decision
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequestResultQueued {
}

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
    /// True, if the link is primary. Primary invite link can't have name, expiration date, or usage limit. Primary link can create join requests only if this is set up using toggleSupergroupJoinByRequest.
    /// There is exactly one primary invite link for each administrator with can_invite_users right at a given time
    pub is_primary: bool,
    /// True, if the link was revoked
    pub is_revoked: bool,
}

/// Contains a list of chat invite links
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinks {
    /// Approximate total number of chat invite links found
    pub total_count: i32,
    /// List of invite links
    pub invite_links: Vec<crate::types::ChatInviteLink>,
}

/// Describes a chat administrator with a number of active and revoked chat invite links
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkCount {
    /// Administrator's user identifier
    pub user_id: i64,
    /// Number of active invite links
    pub invite_link_count: i32,
    /// Number of revoked invite links
    pub revoked_invite_link_count: i32,
}

/// Contains a list of chat invite link counts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkCounts {
    /// List of invite link counts
    pub invite_link_counts: Vec<crate::types::ChatInviteLinkCount>,
}

/// Describes a chat member joined a chat via an invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkMember {
    /// User identifier
    pub user_id: i64,
    /// Point in time (Unix timestamp) when the user joined the chat
    pub joined_chat_date: i32,
    /// True, if the user has joined the chat using an invite link for a chat folder
    pub via_chat_folder_invite_link: bool,
    /// User identifier of the chat administrator, approved user join request
    pub approver_user_id: i64,
}

/// Contains a list of chat members joined a chat via an invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkMembers {
    /// Approximate total number of chat members found
    pub total_count: i32,
    /// List of chat members, joined a chat via an invite link
    pub members: Vec<crate::types::ChatInviteLinkMember>,
}

/// The link is an invite link for a basic group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InviteLinkChatTypeBasicGroup {
}

/// The link is an invite link for a supergroup
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InviteLinkChatTypeSupergroup {
}

/// The link is an invite link for a channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InviteLinkChatTypeChannel {
}

/// Contains information about subscription plan that must be paid by the user to use a chat invite link
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkSubscriptionInfo {
    /// Information about subscription plan that must be paid by the user to use the link
    pub pricing: crate::types::StarSubscriptionPricing,
    /// True, if the user has already paid for the subscription and can use joinChatByInviteLink to join the subscribed chat again
    pub can_reuse: bool,
    /// Identifier of the payment form to use for subscription payment; 0 if the subscription can't be paid
    #[serde_as(as = "DisplayFromStr")]
    pub form_id: i64,
}

/// Contains information about a chat invite link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkInfo {
    /// Chat identifier of the invite link; 0 if the user has no access to the chat before joining
    pub chat_id: i64,
    /// If non-zero, the amount of time for which read access to the chat will remain available, in seconds
    pub accessible_for: i32,
    /// Type of the chat
    #[serde(rename = "type")]
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

/// Describes a user who sent a join request and waits for administrator approval
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequest {
    /// User identifier
    pub user_id: i64,
    /// Point in time (Unix timestamp) when the user sent the join request
    pub date: i32,
    /// A short bio of the user
    pub bio: String,
}

/// Contains a list of requests to join a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequests {
    /// Approximate total number of requests found
    pub total_count: i32,
    /// List of the requests
    pub requests: Vec<crate::types::ChatJoinRequest>,
}

/// Contains information about pending join requests for a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequestsInfo {
    /// Total number of pending join requests
    pub total_count: i32,
    /// Identifiers of at most 3 users sent the newest pending join requests
    pub user_ids: Vec<i64>,
}

/// Represents a supergroup or channel with zero or more members (subscribers in the case of channels)
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
    /// True, if all users directly joining the supergroup need to be approved by supergroup administrators
    pub join_by_request: bool,
    /// True, if the slow mode is enabled in the supergroup
    pub is_slow_mode_enabled: bool,
    /// True, if the supergroup is a channel, which can have an unlimited number of subscribers, but only administrators can post there and see the list of subscribers
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

/// Contains full information about a supergroup or channel
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SupergroupFullInfo {
    /// Chat photo; may be null if empty or unknown. If non-null, then it is the same photo as in chat.photo
    pub photo: Option<crate::types::ChatPhoto>,
    /// Identifier of the community to which the corresponding chat was added
    pub community_id: i64,
    /// Supergroup or channel description
    pub description: String,
    /// Number of members in the supergroup or channel; 0 if unknown
    pub member_count: i32,
    /// Number of privileged users in the supergroup or channel; 0 if unknown
    pub administrator_count: i32,
    /// Number of restricted users in the supergroup; 0 if unknown
    pub restricted_count: i32,
    /// Number of users banned from chat; 0 if unknown
    pub banned_count: i32,
    /// Chat identifier of a discussion group for the channel, or a channel, for which the supergroup is the designated discussion group; 0 if none or unknown
    pub linked_chat_id: i64,
    /// Chat identifier of a direct messages group for the channel, or a channel, for which the supergroup is the designated direct messages group; 0 if none
    pub direct_messages_chat_id: i64,
    /// Delay between consecutive sent messages for non-administrator supergroup members, in seconds
    pub slow_mode_delay: i32,
    /// Time left before next message can be sent in the supergroup, in seconds. An updateSupergroupFullInfo update is not triggered when value of this field changes, but both new and old values are non-zero
    pub slow_mode_delay_expires_in: f64,
    /// True, if paid messages can be enabled in the supergroup chat; for supergroup only
    pub can_enable_paid_messages: bool,
    /// True, if paid reaction can be enabled in the channel chat; for channels only
    pub can_enable_paid_reaction: bool,
    /// True, if members of the chat can be retrieved via getSupergroupMembers or searchChatMembers
    pub can_get_members: bool,
    /// True, if non-administrators can receive only administrators and bots using getSupergroupMembers or searchChatMembers
    pub has_hidden_members: bool,
    /// True, if non-administrators and non-bots can be hidden in responses to getSupergroupMembers and searchChatMembers for non-administrators
    pub can_hide_members: bool,
    /// True, if the supergroup sticker set can be changed
    pub can_set_sticker_set: bool,
    /// True, if the supergroup location can be changed
    pub can_set_location: bool,
    /// True, if the supergroup or channel statistics are available
    pub can_get_statistics: bool,
    /// True, if the supergroup or channel revenue statistics are available
    pub can_get_revenue_statistics: bool,
    /// True, if the supergroup or channel Telegram Star revenue statistics are available
    pub can_get_star_revenue_statistics: bool,
    /// True, if the user can send a gift to the supergroup or channel using sendGift or transferGift
    pub can_send_gift: bool,
    /// True, if aggressive anti-spam checks can be enabled or disabled in the supergroup
    pub can_toggle_aggressive_anti_spam: bool,
    /// True, if new chat members will have access to old messages. In public, discussion, of forum groups and all channels, old messages are always available,
    /// so this option affects only private non-forum supergroups without a linked chat. The value of this field is only available to chat administrators
    pub is_all_history_available: bool,
    /// True, if the chat can have sponsored messages. The value of this field is only available to the owner of the chat
    pub can_have_sponsored_messages: bool,
    /// True, if aggressive anti-spam checks are enabled in the supergroup. The value of this field is only available to chat administrators
    pub has_aggressive_anti_spam_enabled: bool,
    /// True, if paid media can be sent and forwarded to the channel chat; for channels only
    pub has_paid_media_allowed: bool,
    /// True, if the supergroup or channel has pinned stories
    pub has_pinned_stories: bool,
    /// Number of saved to profile gifts for channels without can_post_messages administrator right, otherwise, the total number of received gifts
    pub gift_count: i32,
    /// Number of times the current user boosted the supergroup or channel
    pub my_boost_count: i32,
    /// Number of times the supergroup must be boosted by a user to ignore slow mode and chat permission restrictions; 0 if unspecified
    pub unrestrict_boost_count: i32,
    /// Number of Telegram Stars that must be paid by the current user for each sent message to the supergroup
    pub outgoing_paid_message_star_count: i64,
    /// Identifier of the supergroup sticker set that must be shown before user sticker sets; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_set_id: i64,
    /// Identifier of the custom emoji sticker set that can be used in the supergroup without Telegram Premium subscription; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_sticker_set_id: i64,
    /// Location to which the supergroup is connected; may be null if none
    pub location: Option<crate::types::ChatLocation>,
    /// Primary invite link for the chat; may be null. For chat administrators with can_invite_users right only
    pub invite_link: Option<crate::types::ChatInviteLink>,
    /// User identifier of the guard bot in the group; for chat administrators only
    pub guard_bot_user_id: i64,
    /// List of commands of bots in the group
    pub bot_commands: Vec<crate::types::BotCommands>,
    /// Information about verification status of the supergroup or the channel provided by a bot; may be null if none or unknown
    pub bot_verification: Option<crate::types::BotVerification>,
    /// The main tab chosen by the administrators of the channel; may be null if not chosen manually
    pub main_profile_tab: Option<crate::enums::ProfileTab>,
    /// Identifier of the basic group from which supergroup was upgraded; 0 if none
    pub upgraded_from_basic_group_id: i64,
    /// Identifier of the last message in the basic group from which supergroup was upgraded; 0 if none
    pub upgraded_from_max_message_id: i64,
}

/// The secret chat is not yet created; waiting for the other user to get online
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SecretChatStatePending {
}

/// The secret chat is ready to use
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SecretChatStateReady {
}

/// The secret chat is closed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SecretChatStateClosed {
}

/// Represents a secret chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SecretChat {
    /// Secret chat identifier
    pub id: i32,
    /// Identifier of the chat partner
    pub user_id: i64,
    /// State of the secret chat
    pub state: crate::enums::SecretChatState,
    /// True, if the chat was created by the current user; false otherwise
    pub is_outbound: bool,
    /// Hash of the currently used key for comparison with the hash of the chat partner's key. This is a string of 36 little-endian bytes, which must be split into groups of 2 bits, each denoting a pixel of one of 4 colors FFFFFF, D5E6F3, 2D5775, and 2F99C9.
    /// The pixels must be used to make a 12x12 square image filled from left to right, top to bottom. Alternatively, the first 32 bytes of the hash can be converted to the hexadecimal format and printed as 32 2-digit hex numbers
    pub key_hash: String,
    /// Secret chat layer; determines features supported by the chat partner's application. Nested text entities and underline and strikethrough entities are supported if the layer >= 101,
    /// files bigger than 2000MB are supported if the layer >= 143, spoiler and custom emoji text entities are supported if the layer >= 144
    pub layer: i32,
}

/// Describes a sponsored chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SponsoredChat {
    /// Unique identifier of this result
    pub unique_id: i64,
    /// Chat identifier
    pub chat_id: i64,
    /// Additional optional information about the sponsor to be shown along with the chat
    pub sponsor_info: String,
    /// If non-empty, additional information about the sponsored chat to be shown along with the chat
    pub additional_info: String,
}

/// Contains a list of sponsored chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SponsoredChats {
    /// List of sponsored chats
    pub chats: Vec<crate::types::SponsoredChat>,
}

/// An ordinary chat with a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatTypePrivate {
    /// User identifier
    pub user_id: i64,
}

/// A basic group (a chat with 0-200 other users)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatTypeBasicGroup {
    /// Basic group identifier
    pub basic_group_id: i64,
}

/// A supergroup or channel (with unlimited members)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatTypeSupergroup {
    /// Supergroup or channel identifier
    pub supergroup_id: i64,
    /// True, if the supergroup is a channel
    pub is_channel: bool,
}

/// A secret chat with a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatTypeSecret {
    /// Secret chat identifier
    pub secret_chat_id: i32,
    /// User identifier of the other user in the secret chat
    pub user_id: i64,
}

/// Represents an icon for a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderIcon {
    /// The chosen icon name for short folder representation; one of "All", "Unread", "Unmuted", "Bots", "Channels", "Groups", "Private", "Custom", "Setup", "Cat", "Crown",
    /// "Favorite", "Flower", "Game", "Home", "Love", "Mask", "Party", "Sport", "Study", "Trade", "Travel", "Work", "Airplane", "Book", "Light", "Like", "Money", "Note", "Palette"
    pub name: String,
}

/// Describes name of a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderName {
    /// The text of the chat folder name; 1-12 characters without line feeds. May contain only CustomEmoji entities
    pub text: crate::types::FormattedText,
    /// True, if custom emoji in the name must be animated
    pub animate_custom_emoji: bool,
}

/// Represents a folder for user chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolder {
    /// The name of the folder
    pub name: crate::types::ChatFolderName,
    /// The chosen icon for the chat folder; may be null. If null, use getChatFolderDefaultIconName to get default icon name for the folder
    pub icon: Option<crate::types::ChatFolderIcon>,
    /// The identifier of the chosen color for the chat folder icon; from -1 to 6. If -1, then color is disabled. Can't be changed if folder tags are disabled or the current user doesn't have Telegram Premium subscription
    pub color_id: i32,
    /// True, if at least one link has been created for the folder
    pub is_shareable: bool,
    /// The chat identifiers of pinned chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") pinned and always included non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium
    pub pinned_chat_ids: Vec<i64>,
    /// The chat identifiers of always included chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") pinned and always included non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium
    pub included_chat_ids: Vec<i64>,
    /// The chat identifiers of always excluded chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") always excluded non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium
    pub excluded_chat_ids: Vec<i64>,
    /// True, if muted chats need to be excluded
    pub exclude_muted: bool,
    /// True, if read chats need to be excluded
    pub exclude_read: bool,
    /// True, if archived chats need to be excluded
    pub exclude_archived: bool,
    /// True, if contacts need to be included
    pub include_contacts: bool,
    /// True, if non-contact users need to be included
    pub include_non_contacts: bool,
    /// True, if bots need to be included
    pub include_bots: bool,
    /// True, if basic groups and supergroups need to be included
    pub include_groups: bool,
    /// True, if channels need to be included
    pub include_channels: bool,
}

/// Contains basic information about a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderInfo {
    /// Unique chat folder identifier
    pub id: i32,
    /// The name of the folder
    pub name: crate::types::ChatFolderName,
    /// The chosen or default icon for the chat folder
    pub icon: crate::types::ChatFolderIcon,
    /// The identifier of the chosen color for the chat folder icon; from -1 to 6. If -1, then color is disabled
    pub color_id: i32,
    /// True, if at least one link has been created for the folder
    pub is_shareable: bool,
    /// True, if the chat folder has invite links created by the current user
    pub has_my_invite_links: bool,
}

/// Contains a chat folder invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderInviteLink {
    /// The chat folder invite link
    pub invite_link: String,
    /// Name of the link
    pub name: String,
    /// Identifiers of chats, included in the link
    pub chat_ids: Vec<i64>,
}

/// Represents a list of chat folder invite links
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderInviteLinks {
    /// List of the invite links
    pub invite_links: Vec<crate::types::ChatFolderInviteLink>,
}

/// Contains information about an invite link to a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderInviteLinkInfo {
    /// Basic information about the chat folder; chat folder identifier will be 0 if the user didn't have the chat folder yet
    pub chat_folder_info: crate::types::ChatFolderInfo,
    /// Identifiers of the chats from the link, which aren't added to the folder yet
    pub missing_chat_ids: Vec<i64>,
    /// Identifiers of the chats from the link, which are added to the folder already
    pub added_chat_ids: Vec<i64>,
}

/// Describes a recommended chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RecommendedChatFolder {
    /// The chat folder
    pub folder: crate::types::ChatFolder,
    /// Chat folder description
    pub description: String,
}

/// Contains a list of recommended chat folders
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RecommendedChatFolders {
    /// List of recommended chat folders
    pub chat_folders: Vec<crate::types::RecommendedChatFolder>,
}

/// Contains settings for automatic moving of chats to and from the Archive chat lists
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ArchiveChatListSettings {
    /// True, if new chats from non-contacts will be automatically archived and muted. Can be set to true only if the option "can_archive_and_mute_new_chats_from_unknown_users" is true
    pub archive_and_mute_new_chats_from_unknown_users: bool,
    /// True, if unmuted chats will be kept in the Archive chat list when they get a new message
    pub keep_unmuted_chats_archived: bool,
    /// True, if unmuted chats, that are always included or pinned in a folder, will be kept in the Archive chat list when they get a new message. Ignored if keep_unmuted_chats_archived == true
    pub keep_chats_from_folders_archived: bool,
}

/// A main list of chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatListMain {
}

/// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatListArchive {
}

/// A list of chats added to a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatListFolder {
    /// Chat folder identifier
    pub chat_folder_id: i32,
}

/// Contains a list of chat lists
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatLists {
    /// List of chat lists
    pub chat_lists: Vec<crate::enums::ChatList>,
}

/// The chat is sponsored by the user's MTProxy server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatSourceMtprotoProxy {
}

/// The chat contains a public service announcement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatSourcePublicServiceAnnouncement {
    /// The type of the announcement
    #[serde(rename = "type")]
    pub r#type: String,
    /// The text of the announcement
    pub text: String,
}

/// Describes a position of a chat in a chat list
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatPosition {
    /// The chat list
    pub list: crate::enums::ChatList,
    /// A parameter used to determine order of the chat in the chat list. Chats must be sorted by the pair (order, chat.id) in descending order
    #[serde_as(as = "DisplayFromStr")]
    pub order: i64,
    /// True, if the chat is pinned in the chat list
    pub is_pinned: bool,
    /// Source of the chat in the chat list; may be null
    pub source: Option<crate::enums::ChatSource>,
}

/// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Chat {
    /// Chat unique identifier
    pub id: i64,
    /// Type of the chat
    #[serde(rename = "type")]
    pub r#type: crate::enums::ChatType,
    /// Chat title
    pub title: String,
    /// Chat photo; may be null
    pub photo: Option<crate::types::ChatPhotoInfo>,
    /// Identifier of the accent color for message sender name, and backgrounds of chat photo, reply header, and link preview
    pub accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the reply header and link preview background for messages sent by the chat; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub background_custom_emoji_id: i64,
    /// Color scheme based on an upgraded gift to be used for the chat instead of accent_color_id and background_custom_emoji_id; may be null if none
    pub upgraded_gift_colors: Option<crate::types::UpgradedGiftColors>,
    /// Identifier of the profile accent color for the chat's profile; -1 if none
    pub profile_accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the background of the chat's profile; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub profile_background_custom_emoji_id: i64,
    /// Actions that non-administrator chat members are allowed to take in the chat
    pub permissions: crate::types::ChatPermissions,
    /// Last message in the chat; may be null if none or unknown
    pub last_message: Option<crate::types::Message>,
    /// Positions of the chat in chat lists
    pub positions: Vec<crate::types::ChatPosition>,
    /// Chat lists to which the chat belongs. A chat can have a non-zero position in a chat list even if it doesn't belong to the chat list and have no position in a chat list even if it belongs to the chat list
    pub chat_lists: Vec<crate::enums::ChatList>,
    /// Identifier of a user or chat that is selected to send messages in the chat; may be null if the user can't change message sender
    pub message_sender_id: Option<crate::enums::MessageSender>,
    /// Block list to which the chat is added; may be null if none
    pub block_list: Option<crate::enums::BlockList>,
    /// True, if chat content can't be saved locally, forwarded, or copied
    pub has_protected_content: bool,
    /// True, if translation of all messages in the chat must be suggested to the user
    pub is_translatable: bool,
    /// True, if the chat is marked as unread
    pub is_marked_as_unread: bool,
    /// True, if the chat is a forum supergroup that must be shown in the "View as topics" mode, or Saved Messages chat that must be shown in the "View as chats"
    pub view_as_topics: bool,
    /// True, if the chat has scheduled messages
    pub has_scheduled_messages: bool,
    /// True, if the chat has welcome messages; for chat administrators with can_change_info administrator right only
    pub has_welcome_messages: bool,
    /// True, if the chat messages can be deleted only for the current user while other users will continue to see the messages
    pub can_be_deleted_only_for_self: bool,
    /// True, if the chat messages can be deleted for all users
    pub can_be_deleted_for_all_users: bool,
    /// True, if the chat can be reported to Telegram moderators through reportChat or reportChatPhoto
    pub can_be_reported: bool,
    /// Default value of the disable_notification parameter, used when a message is sent to the chat
    pub default_disable_notification: bool,
    /// Number of unread messages in the chat
    pub unread_count: i32,
    /// Identifier of the last read incoming message
    pub last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing message
    pub last_read_outbox_message_id: i64,
    /// Number of unread messages with a mention/reply in the chat
    pub unread_mention_count: i32,
    /// Number of messages with unread reactions in the chat
    pub unread_reaction_count: i32,
    /// Number of messages with unread poll votes in the chat
    pub unread_poll_vote_count: i32,
    /// Notification settings for the chat
    pub notification_settings: crate::types::ChatNotificationSettings,
    /// Types of reaction, available in the chat
    pub available_reactions: crate::enums::ChatAvailableReactions,
    /// Current message auto-delete or self-destruct timer setting for the chat, in seconds; 0 if disabled. Self-destruct timer in secret chats starts after the message or its content is viewed. Auto-delete timer in other chats starts from the send date
    pub message_auto_delete_time: i32,
    /// Emoji status to be shown along with chat title; may be null
    pub emoji_status: Option<crate::types::EmojiStatus>,
    /// Background set for the chat; may be null if none
    pub background: Option<crate::types::ChatBackground>,
    /// Theme set for the chat; may be null if none
    pub theme: Option<crate::enums::ChatTheme>,
    /// Information about actions which must be possible to do through the chat action bar; may be null if none
    pub action_bar: Option<crate::enums::ChatActionBar>,
    /// Information about bar for managing a business bot in the chat; may be null if none
    pub business_bot_manage_bar: Option<crate::types::BusinessBotManageBar>,
    /// Information about video chat of the chat
    pub video_chat: crate::types::VideoChat,
    /// Information about pending join requests; may be null if none
    pub pending_join_requests: Option<crate::types::ChatJoinRequestsInfo>,
    /// Identifier of the message from which reply markup needs to be used; 0 if there is no reply markup in the chat
    pub reply_markup_message_id: i64,
    /// A draft of a message in the chat; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
    /// Application-specific data associated with the chat. (For example, the chat scroll position or local chat notification settings can be stored here.) Persistent if the message database is used
    pub client_data: String,
}

/// Represents a list of chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Chats {
    /// Approximate total number of chats found
    pub total_count: i32,
    /// List of chat identifiers
    pub chat_ids: Vec<i64>,
}

/// Contains information about a newly created basic group chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreatedBasicGroupChat {
    /// Chat identifier
    pub chat_id: i64,
    /// Information about failed to add members
    pub failed_to_add_members: crate::types::FailedToAddMembers,
}

/// The chat is public, because it has an active username
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PublicChatTypeHasUsername {
}

/// The chat is public, because it is a location-based supergroup
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PublicChatTypeIsLocationBased {
}

/// The chat can be reported as spam using the method reportChat with an empty option_id and message_ids. If the chat is a private chat with a user with an emoji status, then a notice about emoji status usage must be shown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarReportSpam {
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings
    pub can_unarchive: bool,
}

/// The chat is a recently created group chat to which new members can be invited
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarInviteMembers {
}

/// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method setMessageSenderBlockList,
/// or the other user can be added to the contact list using the method addContact. If the chat is a private chat with a user with an emoji status, then a notice about emoji status usage must be shown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarReportAddBlock {
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings
    pub can_unarchive: bool,
    /// Basic information about the other user in the chat; may be null if unknown
    pub account_info: Option<crate::types::AccountInfo>,
}

/// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarAddContact {
}

/// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarSharePhoneNumber {
}

/// The chat is a private chat with an administrator of a chat to which the user sent join request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarJoinRequest {
    /// Title of the chat to which the join request was sent
    pub title: String,
    /// True, if the join request was sent to a channel chat
    pub is_channel: bool,
    /// Point in time (Unix timestamp) when the join request was sent
    pub request_date: i32,
}

/// A button that requests a chat to be shared by the current user; available only in private chats. Use the method shareChatWithBot to complete the request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestChat {
    /// Unique button identifier
    pub id: i32,
    /// True, if the chat must be a channel; otherwise, a basic group or a supergroup chat is shared
    pub chat_is_channel: bool,
    /// True, if the chat must or must not be a forum supergroup
    pub restrict_chat_is_forum: bool,
    /// True, if the chat must be a forum supergroup; otherwise, the chat must not be a forum supergroup. Ignored if restrict_chat_is_forum is false
    pub chat_is_forum: bool,
    /// True, if the chat must or must not have a username
    pub restrict_chat_has_username: bool,
    /// True, if the chat must have a username; otherwise, the chat must not have a username. Ignored if restrict_chat_has_username is false
    pub chat_has_username: bool,
    /// True, if the chat must be created by the current user
    pub chat_is_created: bool,
    /// Expected user administrator rights in the chat; may be null if they aren't restricted
    pub user_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// Expected bot administrator rights in the chat; may be null if they aren't restricted
    pub bot_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// True, if the bot must be a member of the chat; for basic group and supergroup chats only
    pub bot_is_member: bool,
    /// Pass true to request title of the chat; bots only
    pub request_title: bool,
    /// Pass true to request username of the chat; bots only
    pub request_username: bool,
    /// Pass true to request photo of the chat; bots only
    pub request_photo: bool,
}

/// Contains information about a chat shared with a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SharedChat {
    /// Chat identifier
    pub chat_id: i64,
    /// Title of the chat; for bots only
    pub title: String,
    /// Username of the chat; for bots only
    pub username: String,
    /// Photo of the chat; for bots only; may be null
    pub photo: Option<crate::types::Photo>,
}

/// A link to a chat; instant view only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockChatLink {
    /// Chat title
    pub title: String,
    /// Chat photo; may be null
    pub photo: Option<crate::types::ChatPhotoInfo>,
    /// Identifier of the accent color for chat title and background of chat photo
    pub accent_color_id: i32,
    /// Chat username by which all other information about the chat can be resolved
    pub username: String,
}

/// The link is a link to a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeChat {
    /// Type of the chat
    #[serde(rename = "type")]
    pub r#type: crate::enums::InviteLinkChatType,
    /// Photo of the chat; may be null
    pub photo: Option<crate::types::ChatPhoto>,
    /// True, if the link only creates join request
    pub creates_join_request: bool,
}

/// The link is a link to a shareable chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeShareableChatFolder {
}

/// Returns only channel chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SearchChatTypeFilterChannel {
}

/// The user is typing a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionTyping {
}

/// The user is recording a voice note
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionRecordingVoiceNote {
}

/// The user is uploading a voice note
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionUploadingVoiceNote {
    /// Upload progress, as a percentage
    pub progress: i32,
}

/// The user is picking a location or venue to send
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionChoosingLocation {
}

/// The user is picking a contact to send
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionChoosingContact {
}

/// The user has started to play a game
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionStartPlayingGame {
}

/// The user has canceled the previous action
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionCancel {
}

/// Describes allowed types for the target chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TargetChatTypes {
    /// True, if private chats with ordinary users are allowed
    pub allow_user_chats: bool,
    /// True, if private chats with other bots are allowed
    pub allow_bot_chats: bool,
    /// True, if basic group and supergroup chats are allowed
    pub allow_group_chats: bool,
    /// True, if channel chats are allowed
    pub allow_channel_chats: bool,
}

/// The currently opened chat and forum topic must be kept
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TargetChatCurrent {
}

/// The chat needs to be chosen by the user among chats of the specified types
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TargetChatChosen {
    /// Allowed types for the chat
    pub types: crate::types::TargetChatTypes,
}

/// The chat needs to be open with the provided internal link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TargetChatInternalLink {
    /// An internal link pointing to the chat
    pub link: crate::enums::InternalLinkType,
}

/// A new member joined the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberJoined {
}

/// A new member joined the chat via an invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberJoinedByInviteLink {
    /// Invite link used to join the chat
    pub invite_link: crate::types::ChatInviteLink,
    /// True, if the user has joined the chat using an invite link for a chat folder
    pub via_chat_folder_invite_link: bool,
}

/// A new member was accepted to the chat by an administrator
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberJoinedByRequest {
    /// User identifier of the chat administrator, approved user join request
    pub approver_user_id: i64,
    /// Invite link used to join the chat; may be null
    pub invite_link: Option<crate::types::ChatInviteLink>,
}

/// A new chat member was invited
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberInvited {
    /// New member user identifier
    pub user_id: i64,
    /// New member status
    pub status: crate::enums::ChatMemberStatus,
}

/// A member left the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberLeft {
}

/// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberPromoted {
    /// Affected chat member user identifier
    pub user_id: i64,
    /// Previous status of the chat member
    pub old_status: crate::enums::ChatMemberStatus,
    /// New status of the chat member
    pub new_status: crate::enums::ChatMemberStatus,
}

/// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberRestricted {
    /// Affected chat member identifier
    pub member_id: crate::enums::MessageSender,
    /// Previous status of the chat member
    pub old_status: crate::enums::ChatMemberStatus,
    /// New status of the chat member
    pub new_status: crate::enums::ChatMemberStatus,
}

/// A chat member tag has been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberTagChanged {
    /// Affected chat member user identifier
    pub user_id: i64,
    /// Previous tag of the chat member
    pub old_tag: String,
    /// New tag of the chat member
    pub new_tag: String,
}

/// A chat member extended their subscription to the chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberSubscriptionExtended {
    /// Affected chat member user identifier
    pub user_id: i64,
    /// Previous status of the chat member
    pub old_status: crate::enums::ChatMemberStatus,
    /// New status of the chat member
    pub new_status: crate::enums::ChatMemberStatus,
}

/// The chat background was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventBackgroundChanged {
    /// Previous background; may be null if none
    pub old_background: Option<crate::types::ChatBackground>,
    /// New background; may be null if none
    pub new_background: Option<crate::types::ChatBackground>,
}

/// The chat description was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventDescriptionChanged {
    /// Previous chat description
    pub old_description: String,
    /// New chat description
    pub new_description: String,
}

/// The linked chat of a supergroup was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventLinkedChatChanged {
    /// Previous supergroup linked chat identifier
    pub old_linked_chat_id: i64,
    /// New supergroup linked chat identifier
    pub new_linked_chat_id: i64,
}

/// The supergroup location was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventLocationChanged {
    /// Previous location; may be null
    pub old_location: Option<crate::types::ChatLocation>,
    /// New location; may be null
    pub new_location: Option<crate::types::ChatLocation>,
}

/// The chat permissions were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventPermissionsChanged {
    /// Previous chat permissions
    pub old_permissions: crate::types::ChatPermissions,
    /// New chat permissions
    pub new_permissions: crate::types::ChatPermissions,
}

/// The slow_mode_delay setting of a supergroup was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventSlowModeDelayChanged {
    /// Previous value of slow_mode_delay, in seconds
    pub old_slow_mode_delay: i32,
    /// New value of slow_mode_delay, in seconds
    pub new_slow_mode_delay: i32,
}

/// The chat title was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventTitleChanged {
    /// Previous chat title
    pub old_title: String,
    /// New chat title
    pub new_title: String,
}

/// The chat editable username was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventUsernameChanged {
    /// Previous chat username
    pub old_username: String,
    /// New chat username
    pub new_username: String,
}

/// The chat active usernames were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventActiveUsernamesChanged {
    /// Previous list of active usernames
    pub old_usernames: Vec<String>,
    /// New list of active usernames
    pub new_usernames: Vec<String>,
}

/// The chat accent color or background custom emoji were changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventAccentColorChanged {
    /// Previous identifier of chat accent color
    pub old_accent_color_id: i32,
    /// Previous identifier of the custom emoji; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub old_background_custom_emoji_id: i64,
    /// New identifier of chat accent color
    pub new_accent_color_id: i32,
    /// New identifier of the custom emoji; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub new_background_custom_emoji_id: i64,
}

/// The has_protected_content setting of a chat was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventHasProtectedContentToggled {
    /// New value of has_protected_content
    pub has_protected_content: bool,
}

/// The can_invite_users permission of a supergroup chat was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventInvitesToggled {
    /// New value of can_invite_users permission
    pub can_invite_users: bool,
}

/// The has_aggressive_anti_spam_enabled setting of a supergroup was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventHasAggressiveAntiSpamEnabledToggled {
    /// New value of has_aggressive_anti_spam_enabled
    pub has_aggressive_anti_spam_enabled: bool,
}

/// The has_automatic_translation setting of a channel was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventAutomaticTranslationToggled {
    /// New value of has_automatic_translation
    pub has_automatic_translation: bool,
}

/// A chat invite link was edited
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventInviteLinkEdited {
    /// Previous information about the invite link
    pub old_invite_link: crate::types::ChatInviteLink,
    /// New information about the invite link
    pub new_invite_link: crate::types::ChatInviteLink,
}

/// A chat invite link was revoked
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventInviteLinkRevoked {
    /// The invite link
    pub invite_link: crate::types::ChatInviteLink,
}

/// A revoked chat invite link was deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventInviteLinkDeleted {
    /// The invite link
    pub invite_link: crate::types::ChatInviteLink,
}

/// Represents a chat event
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEvent {
    /// Chat event identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Point in time (Unix timestamp) when the event happened
    pub date: i32,
    /// Identifier of the user or chat who performed the action
    pub member_id: crate::enums::MessageSender,
    /// The action
    pub action: crate::enums::ChatEventAction,
}

/// Contains a list of chat events
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEvents {
    /// List of events
    pub events: Vec<crate::types::ChatEvent>,
}

/// Represents a set of filters used to obtain a chat event log
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventLogFilters {
    /// True, if message edits need to be returned
    pub message_edits: bool,
    /// True, if message deletions need to be returned
    pub message_deletions: bool,
    /// True, if pin/unpin events need to be returned
    pub message_pins: bool,
    /// True, if members joining events need to be returned
    pub member_joins: bool,
    /// True, if members leaving events need to be returned
    pub member_leaves: bool,
    /// True, if invited member events need to be returned
    pub member_invites: bool,
    /// True, if member promotion/demotion events need to be returned
    pub member_promotions: bool,
    /// True, if member restricted/unrestricted/banned/unbanned events need to be returned
    pub member_restrictions: bool,
    /// True, if member tag and custom title change events need to be returned
    pub member_tag_changes: bool,
    /// True, if changes in chat information need to be returned
    pub info_changes: bool,
    /// True, if changes in chat settings need to be returned
    pub setting_changes: bool,
    /// True, if changes to invite links need to be returned
    pub invite_link_changes: bool,
    /// True, if video chat actions need to be returned
    pub video_chat_changes: bool,
    /// True, if forum-related actions need to be returned
    pub forum_changes: bool,
    /// True, if subscription extensions need to be returned
    pub subscription_extensions: bool,
}

/// A background from a chat theme based on an emoji; can be used only as a chat background in channels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTypeChatTheme {
    /// Name of the emoji chat theme
    pub theme_name: String,
}

/// Describes a chat theme based on an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftChatTheme {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// Theme settings for a light chat theme
    pub light_settings: crate::types::ThemeSettings,
    /// Theme settings for a dark chat theme
    pub dark_settings: crate::types::ThemeSettings,
}

/// Contains a list of chat themes based on upgraded gifts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftChatThemes {
    /// A list of chat themes
    pub themes: Vec<crate::types::GiftChatTheme>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// A chat theme based on an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatThemeGift {
    /// The chat theme
    pub gift_theme: crate::types::GiftChatTheme,
}

/// A theme based on an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChatThemeGift {
    /// Name of the upgraded gift. A gift can be used only in one chat in a time.
    /// When the same gift is used in another chat, theme in the previous chat is reset to default
    pub name: String,
}

/// The username can be set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckChatUsernameResultOk {
}

/// The username is invalid
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckChatUsernameResultUsernameInvalid {
}

/// The username is occupied
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckChatUsernameResultUsernameOccupied {
}

/// The username can be purchased at https:fragment.com. Information about the username can be received using getCollectibleItemInfo
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckChatUsernameResultUsernamePurchasable {
}

/// The user has too many chats with username, one of them must be made private first
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckChatUsernameResultPublicChatsTooMany {
}

/// The user can't be a member of a public supergroup
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CheckChatUsernameResultPublicGroupsUnavailable {
}

/// A rule to allow all members of certain specified basic groups and supergroups to doing something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleAllowChatMembers {
    /// The chat identifiers, total number of chats in all rules must not exceed 20
    pub chat_ids: Vec<i64>,
}

/// A rule to restrict all members of specified basic groups and supergroups from doing something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleRestrictChatMembers {
    /// The chat identifiers, total number of chats in all rules must not exceed 20
    pub chat_ids: Vec<i64>,
}

/// A privacy setting for managing whether the user can be invited to chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingAllowChatInvites {
}

/// Contains privacy settings for chats with non-contacts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NewChatPrivacySettings {
    /// True, if non-contacts users are able to write first to the current user. Telegram Premium subscribers are able to write first regardless of this setting
    pub allow_new_chats_from_unknown_users: bool,
    /// Number of Telegram Stars that must be paid for every incoming private message by non-contacts; 0-getOption("paid_message_star_count_max").
    /// If positive, then allow_new_chats_from_unknown_users must be true. The current user will receive getOption("paid_message_earnings_per_mille") Telegram Stars for each 1000 Telegram Stars paid for message sending.
    /// Can be positive, only if getOption("can_enable_paid_messages") is true
    pub incoming_paid_message_star_count: i64,
}

/// The chat was reported successfully
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportChatResultOk {
}

/// The user must choose an option to report the chat and repeat request with the chosen option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportChatResultOptionRequired {
    /// Title for the option choice
    pub title: String,
    /// List of available options
    pub options: Vec<crate::types::ReportOption>,
}

/// The chat folder settings section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionChatFolders {
    /// Subsection of the section; may be one of
    /// "", "edit", "create", "add-recommended", "show-tags", "tab-view"
    pub subsection: String,
}

/// The link is an invite link to a chat folder. Call checkChatFolderInviteLink with the given invite link to process the link.
/// If the link is valid and the user wants to join the chat folder, then call addChatFolderByInviteLink
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatFolderInvite {
    /// Internal representation of the invite link
    pub invite_link: String,
}

/// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link.
/// If the link is valid and the user wants to join the chat, then call joinChatByInviteLink
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatInvite {
    /// Internal representation of the invite link
    pub invite_link: String,
}

/// The link is a link that allows to select some chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatSelection {
}

/// The link is a link to the screen for creating a new channel chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeNewChannelChat {
}

/// The link is a link to the screen for creating a new group chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeNewGroupChat {
}

/// The link is a link to the screen for creating a new private chat with a contact
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeNewPrivateChat {
}

/// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link.
/// If the chat is found, open its profile information screen or the chat itself.
/// If draft text isn't empty and the chat is a private chat with a regular user, then put the draft text in the input field
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePublicChat {
    /// Username of the chat
    pub chat_username: String,
    /// Draft text for message to send in the chat
    pub draft_text: String,
    /// True, if chat profile information screen must be opened; otherwise, the chat itself must be opened
    pub open_profile: bool,
}

/// Contains the storage usage statistics for a specific chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorageStatisticsByChat {
    /// Chat identifier; 0 if none
    pub chat_id: i64,
    /// Total size of the files in the chat, in bytes
    pub size: i64,
    /// Total number of files in the chat
    pub count: i32,
    /// Statistics split by file types
    pub by_file_type: Vec<crate::types::StorageStatisticsByFileType>,
}

/// Autosave settings applied to all private chats without chat-specific settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettingsScopePrivateChats {
}

/// Autosave settings applied to all basic group and supergroup chats without chat-specific settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettingsScopeGroupChats {
}

/// Autosave settings applied to all channel chats without chat-specific settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettingsScopeChannelChats {
}

/// Autosave settings applied to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettingsScopeChat {
    /// Chat identifier
    pub chat_id: i64,
}

/// A category containing frequently used private chats with non-bot users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryUsers {
}

/// A category containing frequently used basic groups and supergroups
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryGroups {
}

/// A category containing frequently used channels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryChannels {
}

/// A category containing frequently used chats used to forward messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TopChatCategoryForwardChats {
}

/// A URL linking to a public supergroup or channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TmeUrlTypeSupergroup {
    /// Identifier of the supergroup or channel
    pub supergroup_id: i64,
}

/// A chat invite link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TmeUrlTypeChatInvite {
    /// Information about the chat invite link
    pub info: crate::types::ChatInviteLinkInfo,
}

/// Suggests the user to enable archive_and_mute_new_chats_from_unknown_users setting in archiveChatListSettings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionEnableArchiveAndMuteNewChats {
}

/// Contains statistics about interactions with a message sent in the chat or a story posted on behalf of the chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsInteractionInfo {
    /// Type of the object
    pub object_type: crate::enums::ChatStatisticsObjectType,
    /// Number of times the object was viewed
    pub view_count: i32,
    /// Number of times the object was forwarded
    pub forward_count: i32,
    /// Number of times reactions were added to the object
    pub reaction_count: i32,
}

/// Contains statistics about administrator actions done by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsAdministratorActionsInfo {
    /// Administrator user identifier
    pub user_id: i64,
    /// Number of messages deleted by the administrator
    pub deleted_message_count: i32,
    /// Number of users banned by the administrator
    pub banned_user_count: i32,
    /// Number of users restricted by the administrator
    pub restricted_user_count: i32,
}

/// Contains statistics about number of new members invited by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsInviterInfo {
    /// User identifier
    pub user_id: i64,
    /// Number of new members invited by the user
    pub added_member_count: i32,
}

/// A detailed statistics about a supergroup chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsSupergroup {
    /// A period to which the statistics applies
    pub period: crate::types::DateRange,
    /// Number of members in the chat
    pub member_count: crate::types::StatisticalValue,
    /// Number of messages sent to the chat
    pub message_count: crate::types::StatisticalValue,
    /// Number of users who viewed messages in the chat
    pub viewer_count: crate::types::StatisticalValue,
    /// Number of users who sent messages to the chat
    pub sender_count: crate::types::StatisticalValue,
    /// A graph containing number of members in the chat
    pub member_count_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of members joined and left the chat
    pub join_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of new member joins per source
    pub join_by_source_graph: crate::enums::StatisticalGraph,
    /// A graph containing distribution of active users per language
    pub language_graph: crate::enums::StatisticalGraph,
    /// A graph containing distribution of sent messages by content type
    pub message_content_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of different actions in the chat
    pub action_graph: crate::enums::StatisticalGraph,
    /// A graph containing distribution of message views per hour
    pub day_graph: crate::enums::StatisticalGraph,
    /// A graph containing distribution of message views per day of week
    pub week_graph: crate::enums::StatisticalGraph,
    /// List of users sent most messages in the last week
    pub top_senders: Vec<crate::types::ChatStatisticsMessageSenderInfo>,
    /// List of most active administrators in the last week
    pub top_administrators: Vec<crate::types::ChatStatisticsAdministratorActionsInfo>,
    /// List of most active inviters of new members in the last week
    pub top_inviters: Vec<crate::types::ChatStatisticsInviterInfo>,
}

/// A detailed statistics about a channel chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsChannel {
    /// A period to which the statistics applies
    pub period: crate::types::DateRange,
    /// Number of members in the chat
    pub member_count: crate::types::StatisticalValue,
    /// Mean number of times the recently sent messages were viewed
    pub mean_message_view_count: crate::types::StatisticalValue,
    /// Mean number of times the recently sent messages were shared
    pub mean_message_share_count: crate::types::StatisticalValue,
    /// Mean number of times reactions were added to the recently sent messages
    pub mean_message_reaction_count: crate::types::StatisticalValue,
    /// Mean number of times the recently posted stories were viewed
    pub mean_story_view_count: crate::types::StatisticalValue,
    /// Mean number of times the recently posted stories were shared
    pub mean_story_share_count: crate::types::StatisticalValue,
    /// Mean number of times reactions were added to the recently posted stories
    pub mean_story_reaction_count: crate::types::StatisticalValue,
    /// A percentage of users with enabled notifications for the chat; 0-100
    pub enabled_notifications_percentage: f64,
    /// A graph containing number of members in the chat
    pub member_count_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of members joined and left the chat
    pub join_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of members muted and unmuted the chat
    pub mute_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of message views in a given hour in the last two weeks
    pub view_count_by_hour_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of message views per source
    pub view_count_by_source_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of new member joins per source
    pub join_by_source_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of users viewed chat messages per language
    pub language_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of chat message views and shares
    pub message_interaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of reactions on messages
    pub message_reaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of story views and shares
    pub story_interaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of reactions on stories
    pub story_reaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of views of associated with the chat instant views
    pub instant_view_interaction_graph: crate::enums::StatisticalGraph,
    /// Detailed statistics about number of views and shares of recently sent messages and posted stories
    pub recent_interactions: Vec<crate::types::ChatStatisticsInteractionInfo>,
}

/// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewChat {
    /// The chat
    pub chat: crate::types::Chat,
}

/// The title of a chat was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatTitle {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat title
    pub title: String,
}

/// Chat accent colors have changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatAccentColors {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat accent color identifier
    pub accent_color_id: i32,
    /// The new identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub background_custom_emoji_id: i64,
    /// Color scheme based on an upgraded gift to be used for the chat instead of accent_color_id and background_custom_emoji_id; may be null if none
    pub upgraded_gift_colors: Option<crate::types::UpgradedGiftColors>,
    /// The new chat profile accent color identifier; -1 if none
    pub profile_accent_color_id: i32,
    /// The new identifier of a custom emoji to be shown on the profile background; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub profile_background_custom_emoji_id: i64,
}

/// Chat permissions were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatPermissions {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat permissions
    pub permissions: crate::types::ChatPermissions,
}

/// The position of a chat in a chat list has changed. An updateChatLastMessage or updateChatDraftMessage update might be sent instead of the update
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatPosition {
    /// Chat identifier
    pub chat_id: i64,
    /// New chat position. If new order is 0, then the chat needs to be removed from the list
    pub position: crate::types::ChatPosition,
}

/// A chat was added to a chat list
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatAddedToList {
    /// Chat identifier
    pub chat_id: i64,
    /// The chat list to which the chat was added
    pub chat_list: crate::enums::ChatList,
}

/// A chat was removed from a chat list
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatRemovedFromList {
    /// Chat identifier
    pub chat_id: i64,
    /// The chat list from which the chat was removed
    pub chat_list: crate::enums::ChatList,
}

/// Incoming messages were read or the number of unread messages has been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatReadInbox {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifier of the last read incoming message
    pub last_read_inbox_message_id: i64,
    /// The number of unread messages left in the chat
    pub unread_count: i32,
}

/// Outgoing messages were read
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatReadOutbox {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifier of last read outgoing message
    pub last_read_outbox_message_id: i64,
}

/// The chat action bar was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatActionBar {
    /// Chat identifier
    pub chat_id: i64,
    /// The new value of the action bar; may be null
    pub action_bar: Option<crate::enums::ChatActionBar>,
}

/// The chat pending join requests were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatPendingJoinRequests {
    /// Chat identifier
    pub chat_id: i64,
    /// The new data about pending join requests; may be null
    pub pending_join_requests: Option<crate::types::ChatJoinRequestsInfo>,
}

/// The chat reply markup was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatReplyMarkup {
    /// Chat identifier
    pub chat_id: i64,
    /// The message from which the reply markup must be used; may be null if there is no default reply markup in the chat
    pub reply_markup_message: Option<crate::types::Message>,
}

/// The chat background was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatBackground {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat background; may be null if background was reset to default
    pub background: Option<crate::types::ChatBackground>,
}

/// The chat theme was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatTheme {
    /// Chat identifier
    pub chat_id: i64,
    /// The new theme of the chat; may be null if theme was reset to default
    pub theme: Option<crate::enums::ChatTheme>,
}

/// The chat unread_mention_count has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatUnreadMentionCount {
    /// Chat identifier
    pub chat_id: i64,
    /// The number of unread mention messages left in the chat
    pub unread_mention_count: i32,
}

/// A chat content was allowed or restricted for saving
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatHasProtectedContent {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of has_protected_content
    pub has_protected_content: bool,
}

/// Translation of chat messages was enabled or disabled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatIsTranslatable {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of is_translatable
    pub is_translatable: bool,
}

/// A chat was marked as unread or was read
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatIsMarkedAsUnread {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of is_marked_as_unread
    pub is_marked_as_unread: bool,
}

/// A chat was blocked or unblocked
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatBlockList {
    /// Chat identifier
    pub chat_id: i64,
    /// Block list to which the chat is added; may be null if none
    pub block_list: Option<crate::enums::BlockList>,
}

/// The list of chat folders or a chat folder has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatFolders {
    /// The new list of chat folders
    pub chat_folders: Vec<crate::types::ChatFolderInfo>,
    /// Position of the main chat list among chat folders, 0-based
    pub main_chat_list_position: i32,
    /// True, if folder tags are enabled
    pub are_tags_enabled: bool,
}

/// The number of online group members has changed. This update with non-zero number of online group members is sent only for currently opened chats.
/// There is no guarantee that it is sent just after the number of online users has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatOnlineMemberCount {
    /// Identifier of the chat
    pub chat_id: i64,
    /// New number of online members in the chat, or 0 if unknown
    pub online_member_count: i32,
}

/// A message sender activity in the chat has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatAction {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifier of the specific topic in which the action was performed; may be null if none
    pub topic_id: Option<crate::enums::MessageTopic>,
    /// Identifier of a message sender performing the action
    pub sender_id: crate::enums::MessageSender,
    /// The action
    pub action: crate::enums::ChatAction,
}

/// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateSupergroup {
    /// New data about the supergroup
    pub supergroup: crate::types::Supergroup,
}

/// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateSecretChat {
    /// New data about the secret chat
    pub secret_chat: crate::types::SecretChat,
}

/// Some data in supergroupFullInfo has been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateSupergroupFullInfo {
    /// Identifier of the supergroup or channel
    pub supergroup_id: i64,
    /// New full information about the supergroup
    pub supergroup_full_info: crate::types::SupergroupFullInfo,
}

/// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUnreadChatCount {
    /// The chat list with changed number of unread messages
    pub chat_list: crate::enums::ChatList,
    /// Approximate total number of chats in the chat list
    pub total_count: i32,
    /// Total number of unread chats
    pub unread_count: i32,
    /// Total number of unread unmuted chats
    pub unread_unmuted_count: i32,
    /// Total number of chats marked as unread
    pub marked_as_unread_count: i32,
    /// Total number of unmuted chats marked as unread
    pub marked_as_unread_unmuted_count: i32,
}

/// A join request from the user was completed
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatJoinResult {
    /// Identifier of the join request query as received in chatJoinResultGuardBotApprovalRequired. If the corresponding Web App is still open, then it must be closed
    #[serde_as(as = "DisplayFromStr")]
    pub query_id: i64,
    /// Identifier of the joined chat, or 0 if the request wasn't approved
    pub chat_id: i64,
    /// Result of the join
    pub result: crate::enums::ChatJoinRequestResult,
}

/// User rights changed in a chat; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatMember {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifier of the user, changing the rights
    pub actor_user_id: i64,
    /// Point in time (Unix timestamp) when the user rights were changed
    pub date: i32,
    /// If user has joined the chat using an invite link, the invite link; may be null
    pub invite_link: Option<crate::types::ChatInviteLink>,
    /// True, if the user has joined the chat after sending a join request and being approved by an administrator
    pub via_join_request: bool,
    /// True, if the user has joined the chat using an invite link for a chat folder
    pub via_chat_folder_invite_link: bool,
    /// Previous chat member
    pub old_chat_member: crate::types::ChatMember,
    /// New chat member
    pub new_chat_member: crate::types::ChatMember,
}

/// A user sent a join request to a chat; for bots only
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewChatJoinRequest {
    /// Chat identifier
    pub chat_id: i64,
    /// Join request
    pub request: crate::types::ChatJoinRequest,
    /// Chat identifier of the private chat with the user
    pub user_chat_id: i64,
    /// The invite link, which was used to send join request; may be null
    pub invite_link: Option<crate::types::ChatInviteLink>,
    /// Identifier of the join request query, which can be used in answerChatJoinRequestQuery; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub query_id: i64,
}


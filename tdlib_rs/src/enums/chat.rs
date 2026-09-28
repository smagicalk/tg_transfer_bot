//!
//! TDLib `chat` domain enums.
//!
//! Types, enums, and functions for managing private chats, basic groups, supergroups, and channels.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// TDLib `ChatBackground` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBackground {
    /// Describes a background set for a specific chat
    #[serde(rename(serialize = "chatBackground", deserialize = "chatBackground"))]
    ChatBackground(Box<crate::types::ChatBackground>),
}

impl ChatBackground {
    /// Convenience constructor to create a [`ChatBackground::ChatBackground`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_background(val: crate::types::ChatBackground) -> Self {
        Self::ChatBackground(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatBackground`] into [`ChatBackground`].
impl From<crate::types::ChatBackground> for ChatBackground {
    fn from(val: crate::types::ChatBackground) -> Self {
        Self::ChatBackground(Box::new(val))
    }
}

/// TDLib `ChatLocation` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatLocation {
    /// Represents a location to which a chat is connected
    #[serde(rename(serialize = "chatLocation", deserialize = "chatLocation"))]
    ChatLocation(Box<crate::types::ChatLocation>),
}

impl ChatLocation {
    /// Convenience constructor to create a [`ChatLocation::ChatLocation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_location(val: crate::types::ChatLocation) -> Self {
        Self::ChatLocation(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatLocation`] into [`ChatLocation`].
impl From<crate::types::ChatLocation> for ChatLocation {
    fn from(val: crate::types::ChatLocation) -> Self {
        Self::ChatLocation(Box::new(val))
    }
}

/// TDLib `ChatPermissions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPermissions {
    /// Describes actions that a user is allowed to take in a chat
    #[serde(rename(serialize = "chatPermissions", deserialize = "chatPermissions"))]
    ChatPermissions(Box<crate::types::ChatPermissions>),
}

impl ChatPermissions {
    /// Convenience constructor to create a [`ChatPermissions::ChatPermissions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_permissions(val: crate::types::ChatPermissions) -> Self {
        Self::ChatPermissions(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatPermissions`] into [`ChatPermissions`].
impl From<crate::types::ChatPermissions> for ChatPermissions {
    fn from(val: crate::types::ChatPermissions) -> Self {
        Self::ChatPermissions(Box::new(val))
    }
}

/// TDLib `ChatAdministratorRights` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAdministratorRights {
    /// Describes rights of the administrator
    #[serde(rename(serialize = "chatAdministratorRights", deserialize = "chatAdministratorRights"))]
    ChatAdministratorRights(Box<crate::types::ChatAdministratorRights>),
}

impl ChatAdministratorRights {
    /// Convenience constructor to create a [`ChatAdministratorRights::ChatAdministratorRights`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_administrator_rights(val: crate::types::ChatAdministratorRights) -> Self {
        Self::ChatAdministratorRights(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatAdministratorRights`] into [`ChatAdministratorRights`].
impl From<crate::types::ChatAdministratorRights> for ChatAdministratorRights {
    fn from(val: crate::types::ChatAdministratorRights) -> Self {
        Self::ChatAdministratorRights(Box::new(val))
    }
}

/// TDLib `CommunityChat` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CommunityChat {
    /// Describes a chat in a community
    #[serde(rename(serialize = "communityChat", deserialize = "communityChat"))]
    CommunityChat(Box<crate::types::CommunityChat>),
}

impl CommunityChat {
    /// Convenience constructor to create a [`CommunityChat::CommunityChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community_chat(val: crate::types::CommunityChat) -> Self {
        Self::CommunityChat(Box::new(val))
    }

}

/// Converts a [`crate::types::CommunityChat`] into [`CommunityChat`].
impl From<crate::types::CommunityChat> for CommunityChat {
    fn from(val: crate::types::CommunityChat) -> Self {
        Self::CommunityChat(Box::new(val))
    }
}

/// TDLib `ChatAdministrator` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAdministrator {
    /// Contains information about a chat administrator
    #[serde(rename(serialize = "chatAdministrator", deserialize = "chatAdministrator"))]
    ChatAdministrator(Box<crate::types::ChatAdministrator>),
}

impl ChatAdministrator {
    /// Convenience constructor to create a [`ChatAdministrator::ChatAdministrator`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_administrator(val: crate::types::ChatAdministrator) -> Self {
        Self::ChatAdministrator(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatAdministrator`] into [`ChatAdministrator`].
impl From<crate::types::ChatAdministrator> for ChatAdministrator {
    fn from(val: crate::types::ChatAdministrator) -> Self {
        Self::ChatAdministrator(Box::new(val))
    }
}

/// TDLib `ChatAdministrators` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAdministrators {
    /// Represents a list of chat administrators
    #[serde(rename(serialize = "chatAdministrators", deserialize = "chatAdministrators"))]
    ChatAdministrators(Box<crate::types::ChatAdministrators>),
}

impl ChatAdministrators {
    /// Convenience constructor to create a [`ChatAdministrators::ChatAdministrators`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_administrators(val: crate::types::ChatAdministrators) -> Self {
        Self::ChatAdministrators(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatAdministrators`] into [`ChatAdministrators`].
impl From<crate::types::ChatAdministrators> for ChatAdministrators {
    fn from(val: crate::types::ChatAdministrators) -> Self {
        Self::ChatAdministrators(Box::new(val))
    }
}

/// Provides information about the status of a member in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMemberStatus {
    /// The user is the owner of the chat and has all the administrator privileges
    #[serde(rename(serialize = "chatMemberStatusCreator", deserialize = "chatMemberStatusCreator"))]
    Creator(Box<crate::types::ChatMemberStatusCreator>),
    /// The user is a member of the chat and has some additional privileges. In basic groups, administrators have all applicable rights.
    /// In supergroups and channels, any subset of the rights can be chosen for an administrator
    #[serde(rename(serialize = "chatMemberStatusAdministrator", deserialize = "chatMemberStatusAdministrator"))]
    Administrator(Box<crate::types::ChatMemberStatusAdministrator>),
    /// The user is a member of the chat, without any additional privileges or restrictions
    #[serde(rename(serialize = "chatMemberStatusMember", deserialize = "chatMemberStatusMember"))]
    Member(Box<crate::types::ChatMemberStatusMember>),
    /// The user is under certain restrictions in the chat. Not supported in basic groups and channels
    #[serde(rename(serialize = "chatMemberStatusRestricted", deserialize = "chatMemberStatusRestricted"))]
    Restricted(Box<crate::types::ChatMemberStatusRestricted>),
    /// The user or the chat is not a chat member
    #[serde(rename(serialize = "chatMemberStatusLeft", deserialize = "chatMemberStatusLeft"))]
    Left,
    /// The user or the chat was banned (and hence is not a member of the chat). Implies the user can't return to the chat, view messages, or be used as a participant identifier to join a video chat of the chat
    #[serde(rename(serialize = "chatMemberStatusBanned", deserialize = "chatMemberStatusBanned"))]
    Banned(Box<crate::types::ChatMemberStatusBanned>),
}

impl ChatMemberStatus {
    /// Convenience constructor to create a [`ChatMemberStatus::Creator`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn creator(val: crate::types::ChatMemberStatusCreator) -> Self {
        Self::Creator(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatMemberStatus::Administrator`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn administrator(val: crate::types::ChatMemberStatusAdministrator) -> Self {
        Self::Administrator(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatMemberStatus::Member`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn member(val: crate::types::ChatMemberStatusMember) -> Self {
        Self::Member(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatMemberStatus::Restricted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn restricted(val: crate::types::ChatMemberStatusRestricted) -> Self {
        Self::Restricted(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatMemberStatus::Banned`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn banned(val: crate::types::ChatMemberStatusBanned) -> Self {
        Self::Banned(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatMemberStatusCreator`] into [`ChatMemberStatus`].
impl From<crate::types::ChatMemberStatusCreator> for ChatMemberStatus {
    fn from(val: crate::types::ChatMemberStatusCreator) -> Self {
        Self::Creator(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatMemberStatusAdministrator`] into [`ChatMemberStatus`].
impl From<crate::types::ChatMemberStatusAdministrator> for ChatMemberStatus {
    fn from(val: crate::types::ChatMemberStatusAdministrator) -> Self {
        Self::Administrator(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatMemberStatusMember`] into [`ChatMemberStatus`].
impl From<crate::types::ChatMemberStatusMember> for ChatMemberStatus {
    fn from(val: crate::types::ChatMemberStatusMember) -> Self {
        Self::Member(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatMemberStatusRestricted`] into [`ChatMemberStatus`].
impl From<crate::types::ChatMemberStatusRestricted> for ChatMemberStatus {
    fn from(val: crate::types::ChatMemberStatusRestricted) -> Self {
        Self::Restricted(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatMemberStatusBanned`] into [`ChatMemberStatus`].
impl From<crate::types::ChatMemberStatusBanned> for ChatMemberStatus {
    fn from(val: crate::types::ChatMemberStatusBanned) -> Self {
        Self::Banned(Box::new(val))
    }
}

/// TDLib `ChatMember` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMember {
    /// Describes a user or a chat as a member of another chat
    #[serde(rename(serialize = "chatMember", deserialize = "chatMember"))]
    ChatMember(Box<crate::types::ChatMember>),
}

impl ChatMember {
    /// Convenience constructor to create a [`ChatMember::ChatMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_member(val: crate::types::ChatMember) -> Self {
        Self::ChatMember(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatMember`] into [`ChatMember`].
impl From<crate::types::ChatMember> for ChatMember {
    fn from(val: crate::types::ChatMember) -> Self {
        Self::ChatMember(Box::new(val))
    }
}

/// TDLib `ChatMembers` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMembers {
    /// Contains a list of chat members
    #[serde(rename(serialize = "chatMembers", deserialize = "chatMembers"))]
    ChatMembers(Box<crate::types::ChatMembers>),
}

impl ChatMembers {
    /// Convenience constructor to create a [`ChatMembers::ChatMembers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_members(val: crate::types::ChatMembers) -> Self {
        Self::ChatMembers(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatMembers`] into [`ChatMembers`].
impl From<crate::types::ChatMembers> for ChatMembers {
    fn from(val: crate::types::ChatMembers) -> Self {
        Self::ChatMembers(Box::new(val))
    }
}

/// Specifies the kind of chat members to return in searchChatMembers
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMembersFilter {
    /// Returns contacts of the user
    #[serde(rename(serialize = "chatMembersFilterContacts", deserialize = "chatMembersFilterContacts"))]
    Contacts,
    /// Returns the owner and administrators
    #[serde(rename(serialize = "chatMembersFilterAdministrators", deserialize = "chatMembersFilterAdministrators"))]
    Administrators,
    /// Returns all chat members, including restricted chat members
    #[serde(rename(serialize = "chatMembersFilterMembers", deserialize = "chatMembersFilterMembers"))]
    Members,
    /// Returns users who can be mentioned in the chat
    #[serde(rename(serialize = "chatMembersFilterMention", deserialize = "chatMembersFilterMention"))]
    Mention(Box<crate::types::ChatMembersFilterMention>),
    /// Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup
    #[serde(rename(serialize = "chatMembersFilterRestricted", deserialize = "chatMembersFilterRestricted"))]
    Restricted,
    /// Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel
    #[serde(rename(serialize = "chatMembersFilterBanned", deserialize = "chatMembersFilterBanned"))]
    Banned,
    /// Returns bot members of the chat
    #[serde(rename(serialize = "chatMembersFilterBots", deserialize = "chatMembersFilterBots"))]
    Bots,
}

impl ChatMembersFilter {
    /// Convenience constructor to create a [`ChatMembersFilter::Mention`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mention(val: crate::types::ChatMembersFilterMention) -> Self {
        Self::Mention(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatMembersFilterMention`] into [`ChatMembersFilter`].
impl From<crate::types::ChatMembersFilterMention> for ChatMembersFilter {
    fn from(val: crate::types::ChatMembersFilterMention) -> Self {
        Self::Mention(Box::new(val))
    }
}

/// Specifies the kind of chat members to return in getSupergroupMembers
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SupergroupMembersFilter {
    /// Returns recently active users in reverse chronological order
    #[serde(rename(serialize = "supergroupMembersFilterRecent", deserialize = "supergroupMembersFilterRecent"))]
    Recent,
    /// Returns contacts of the current user who are members of the supergroup or channel
    #[serde(rename(serialize = "supergroupMembersFilterContacts", deserialize = "supergroupMembersFilterContacts"))]
    Contacts(Box<crate::types::SupergroupMembersFilterContacts>),
    /// Returns the owner and administrators
    #[serde(rename(serialize = "supergroupMembersFilterAdministrators", deserialize = "supergroupMembersFilterAdministrators"))]
    Administrators,
    /// Used to search for supergroup or channel members via a (string) query
    #[serde(rename(serialize = "supergroupMembersFilterSearch", deserialize = "supergroupMembersFilterSearch"))]
    Search(Box<crate::types::SupergroupMembersFilterSearch>),
    /// Returns restricted supergroup members; can be used only by administrators
    #[serde(rename(serialize = "supergroupMembersFilterRestricted", deserialize = "supergroupMembersFilterRestricted"))]
    Restricted(Box<crate::types::SupergroupMembersFilterRestricted>),
    /// Returns users banned from the supergroup or channel; can be used only by administrators
    #[serde(rename(serialize = "supergroupMembersFilterBanned", deserialize = "supergroupMembersFilterBanned"))]
    Banned(Box<crate::types::SupergroupMembersFilterBanned>),
    /// Returns users who can be mentioned in the supergroup
    #[serde(rename(serialize = "supergroupMembersFilterMention", deserialize = "supergroupMembersFilterMention"))]
    Mention(Box<crate::types::SupergroupMembersFilterMention>),
    /// Returns bot members of the supergroup or channel
    #[serde(rename(serialize = "supergroupMembersFilterBots", deserialize = "supergroupMembersFilterBots"))]
    Bots,
}

impl SupergroupMembersFilter {
    /// Convenience constructor to create a [`SupergroupMembersFilter::Contacts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contacts(val: crate::types::SupergroupMembersFilterContacts) -> Self {
        Self::Contacts(Box::new(val))
    }

    /// Convenience constructor to create a [`SupergroupMembersFilter::Search`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn search(val: crate::types::SupergroupMembersFilterSearch) -> Self {
        Self::Search(Box::new(val))
    }

    /// Convenience constructor to create a [`SupergroupMembersFilter::Restricted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn restricted(val: crate::types::SupergroupMembersFilterRestricted) -> Self {
        Self::Restricted(Box::new(val))
    }

    /// Convenience constructor to create a [`SupergroupMembersFilter::Banned`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn banned(val: crate::types::SupergroupMembersFilterBanned) -> Self {
        Self::Banned(Box::new(val))
    }

    /// Convenience constructor to create a [`SupergroupMembersFilter::Mention`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mention(val: crate::types::SupergroupMembersFilterMention) -> Self {
        Self::Mention(Box::new(val))
    }

}

/// Converts a [`crate::types::SupergroupMembersFilterContacts`] into [`SupergroupMembersFilter`].
impl From<crate::types::SupergroupMembersFilterContacts> for SupergroupMembersFilter {
    fn from(val: crate::types::SupergroupMembersFilterContacts) -> Self {
        Self::Contacts(Box::new(val))
    }
}

/// Converts a [`crate::types::SupergroupMembersFilterSearch`] into [`SupergroupMembersFilter`].
impl From<crate::types::SupergroupMembersFilterSearch> for SupergroupMembersFilter {
    fn from(val: crate::types::SupergroupMembersFilterSearch) -> Self {
        Self::Search(Box::new(val))
    }
}

/// Converts a [`crate::types::SupergroupMembersFilterRestricted`] into [`SupergroupMembersFilter`].
impl From<crate::types::SupergroupMembersFilterRestricted> for SupergroupMembersFilter {
    fn from(val: crate::types::SupergroupMembersFilterRestricted) -> Self {
        Self::Restricted(Box::new(val))
    }
}

/// Converts a [`crate::types::SupergroupMembersFilterBanned`] into [`SupergroupMembersFilter`].
impl From<crate::types::SupergroupMembersFilterBanned> for SupergroupMembersFilter {
    fn from(val: crate::types::SupergroupMembersFilterBanned) -> Self {
        Self::Banned(Box::new(val))
    }
}

/// Converts a [`crate::types::SupergroupMembersFilterMention`] into [`SupergroupMembersFilter`].
impl From<crate::types::SupergroupMembersFilterMention> for SupergroupMembersFilter {
    fn from(val: crate::types::SupergroupMembersFilterMention) -> Self {
        Self::Mention(Box::new(val))
    }
}

/// Describes result of join of a chat by the current user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatJoinResult {
    /// The chat was joined successfully
    #[serde(rename(serialize = "chatJoinResultSuccess", deserialize = "chatJoinResultSuccess"))]
    Success(Box<crate::types::ChatJoinResultSuccess>),
    /// The join request was sent and have to be approved by administrators of the chat
    #[serde(rename(serialize = "chatJoinResultRequestSent", deserialize = "chatJoinResultRequestSent"))]
    RequestSent,
    /// An approval from a guard bot through a Web App is required to join the chat
    #[serde(rename(serialize = "chatJoinResultGuardBotApprovalRequired", deserialize = "chatJoinResultGuardBotApprovalRequired"))]
    GuardBotApprovalRequired(Box<crate::types::ChatJoinResultGuardBotApprovalRequired>),
    /// The join was declined by the guard bot
    #[serde(rename(serialize = "chatJoinResultDeclined", deserialize = "chatJoinResultDeclined"))]
    Declined,
}

impl ChatJoinResult {
    /// Convenience constructor to create a [`ChatJoinResult::Success`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn success(val: crate::types::ChatJoinResultSuccess) -> Self {
        Self::Success(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatJoinResult::GuardBotApprovalRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn guard_bot_approval_required(val: crate::types::ChatJoinResultGuardBotApprovalRequired) -> Self {
        Self::GuardBotApprovalRequired(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatJoinResultSuccess`] into [`ChatJoinResult`].
impl From<crate::types::ChatJoinResultSuccess> for ChatJoinResult {
    fn from(val: crate::types::ChatJoinResultSuccess) -> Self {
        Self::Success(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatJoinResultGuardBotApprovalRequired`] into [`ChatJoinResult`].
impl From<crate::types::ChatJoinResultGuardBotApprovalRequired> for ChatJoinResult {
    fn from(val: crate::types::ChatJoinResultGuardBotApprovalRequired) -> Self {
        Self::GuardBotApprovalRequired(Box::new(val))
    }
}

/// Describes result of a chat join request
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatJoinRequestResult {
    /// The request was approved
    #[serde(rename(serialize = "chatJoinRequestResultApproved", deserialize = "chatJoinRequestResultApproved"))]
    Approved,
    /// The request was declined
    #[serde(rename(serialize = "chatJoinRequestResultDeclined", deserialize = "chatJoinRequestResultDeclined"))]
    Declined,
    /// The request was postponed without a decision
    #[serde(rename(serialize = "chatJoinRequestResultQueued", deserialize = "chatJoinRequestResultQueued"))]
    Queued,
}

/// TDLib `ChatInviteLink` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLink {
    /// Contains a chat invite link
    #[serde(rename(serialize = "chatInviteLink", deserialize = "chatInviteLink"))]
    ChatInviteLink(Box<crate::types::ChatInviteLink>),
}

impl ChatInviteLink {
    /// Convenience constructor to create a [`ChatInviteLink::ChatInviteLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite_link(val: crate::types::ChatInviteLink) -> Self {
        Self::ChatInviteLink(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatInviteLink`] into [`ChatInviteLink`].
impl From<crate::types::ChatInviteLink> for ChatInviteLink {
    fn from(val: crate::types::ChatInviteLink) -> Self {
        Self::ChatInviteLink(Box::new(val))
    }
}

/// TDLib `ChatInviteLinks` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinks {
    /// Contains a list of chat invite links
    #[serde(rename(serialize = "chatInviteLinks", deserialize = "chatInviteLinks"))]
    ChatInviteLinks(Box<crate::types::ChatInviteLinks>),
}

impl ChatInviteLinks {
    /// Convenience constructor to create a [`ChatInviteLinks::ChatInviteLinks`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite_links(val: crate::types::ChatInviteLinks) -> Self {
        Self::ChatInviteLinks(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatInviteLinks`] into [`ChatInviteLinks`].
impl From<crate::types::ChatInviteLinks> for ChatInviteLinks {
    fn from(val: crate::types::ChatInviteLinks) -> Self {
        Self::ChatInviteLinks(Box::new(val))
    }
}

/// TDLib `ChatInviteLinkCount` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkCount {
    /// Describes a chat administrator with a number of active and revoked chat invite links
    #[serde(rename(serialize = "chatInviteLinkCount", deserialize = "chatInviteLinkCount"))]
    ChatInviteLinkCount(Box<crate::types::ChatInviteLinkCount>),
}

impl ChatInviteLinkCount {
    /// Convenience constructor to create a [`ChatInviteLinkCount::ChatInviteLinkCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite_link_count(val: crate::types::ChatInviteLinkCount) -> Self {
        Self::ChatInviteLinkCount(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatInviteLinkCount`] into [`ChatInviteLinkCount`].
impl From<crate::types::ChatInviteLinkCount> for ChatInviteLinkCount {
    fn from(val: crate::types::ChatInviteLinkCount) -> Self {
        Self::ChatInviteLinkCount(Box::new(val))
    }
}

/// TDLib `ChatInviteLinkCounts` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkCounts {
    /// Contains a list of chat invite link counts
    #[serde(rename(serialize = "chatInviteLinkCounts", deserialize = "chatInviteLinkCounts"))]
    ChatInviteLinkCounts(Box<crate::types::ChatInviteLinkCounts>),
}

impl ChatInviteLinkCounts {
    /// Convenience constructor to create a [`ChatInviteLinkCounts::ChatInviteLinkCounts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite_link_counts(val: crate::types::ChatInviteLinkCounts) -> Self {
        Self::ChatInviteLinkCounts(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatInviteLinkCounts`] into [`ChatInviteLinkCounts`].
impl From<crate::types::ChatInviteLinkCounts> for ChatInviteLinkCounts {
    fn from(val: crate::types::ChatInviteLinkCounts) -> Self {
        Self::ChatInviteLinkCounts(Box::new(val))
    }
}

/// TDLib `ChatInviteLinkMember` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkMember {
    /// Describes a chat member joined a chat via an invite link
    #[serde(rename(serialize = "chatInviteLinkMember", deserialize = "chatInviteLinkMember"))]
    ChatInviteLinkMember(Box<crate::types::ChatInviteLinkMember>),
}

impl ChatInviteLinkMember {
    /// Convenience constructor to create a [`ChatInviteLinkMember::ChatInviteLinkMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite_link_member(val: crate::types::ChatInviteLinkMember) -> Self {
        Self::ChatInviteLinkMember(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatInviteLinkMember`] into [`ChatInviteLinkMember`].
impl From<crate::types::ChatInviteLinkMember> for ChatInviteLinkMember {
    fn from(val: crate::types::ChatInviteLinkMember) -> Self {
        Self::ChatInviteLinkMember(Box::new(val))
    }
}

/// TDLib `ChatInviteLinkMembers` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkMembers {
    /// Contains a list of chat members joined a chat via an invite link
    #[serde(rename(serialize = "chatInviteLinkMembers", deserialize = "chatInviteLinkMembers"))]
    ChatInviteLinkMembers(Box<crate::types::ChatInviteLinkMembers>),
}

impl ChatInviteLinkMembers {
    /// Convenience constructor to create a [`ChatInviteLinkMembers::ChatInviteLinkMembers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite_link_members(val: crate::types::ChatInviteLinkMembers) -> Self {
        Self::ChatInviteLinkMembers(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatInviteLinkMembers`] into [`ChatInviteLinkMembers`].
impl From<crate::types::ChatInviteLinkMembers> for ChatInviteLinkMembers {
    fn from(val: crate::types::ChatInviteLinkMembers) -> Self {
        Self::ChatInviteLinkMembers(Box::new(val))
    }
}

/// Describes the type of chat to which points an invite link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InviteLinkChatType {
    /// The link is an invite link for a basic group
    #[serde(rename(serialize = "inviteLinkChatTypeBasicGroup", deserialize = "inviteLinkChatTypeBasicGroup"))]
    BasicGroup,
    /// The link is an invite link for a supergroup
    #[serde(rename(serialize = "inviteLinkChatTypeSupergroup", deserialize = "inviteLinkChatTypeSupergroup"))]
    Supergroup,
    /// The link is an invite link for a channel
    #[serde(rename(serialize = "inviteLinkChatTypeChannel", deserialize = "inviteLinkChatTypeChannel"))]
    Channel,
}

/// TDLib `ChatInviteLinkSubscriptionInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkSubscriptionInfo {
    /// Contains information about subscription plan that must be paid by the user to use a chat invite link
    #[serde(rename(serialize = "chatInviteLinkSubscriptionInfo", deserialize = "chatInviteLinkSubscriptionInfo"))]
    ChatInviteLinkSubscriptionInfo(Box<crate::types::ChatInviteLinkSubscriptionInfo>),
}

impl ChatInviteLinkSubscriptionInfo {
    /// Convenience constructor to create a [`ChatInviteLinkSubscriptionInfo::ChatInviteLinkSubscriptionInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite_link_subscription_info(val: crate::types::ChatInviteLinkSubscriptionInfo) -> Self {
        Self::ChatInviteLinkSubscriptionInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatInviteLinkSubscriptionInfo`] into [`ChatInviteLinkSubscriptionInfo`].
impl From<crate::types::ChatInviteLinkSubscriptionInfo> for ChatInviteLinkSubscriptionInfo {
    fn from(val: crate::types::ChatInviteLinkSubscriptionInfo) -> Self {
        Self::ChatInviteLinkSubscriptionInfo(Box::new(val))
    }
}

/// TDLib `ChatInviteLinkInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatInviteLinkInfo {
    /// Contains information about a chat invite link
    #[serde(rename(serialize = "chatInviteLinkInfo", deserialize = "chatInviteLinkInfo"))]
    ChatInviteLinkInfo(Box<crate::types::ChatInviteLinkInfo>),
}

impl ChatInviteLinkInfo {
    /// Convenience constructor to create a [`ChatInviteLinkInfo::ChatInviteLinkInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite_link_info(val: crate::types::ChatInviteLinkInfo) -> Self {
        Self::ChatInviteLinkInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatInviteLinkInfo`] into [`ChatInviteLinkInfo`].
impl From<crate::types::ChatInviteLinkInfo> for ChatInviteLinkInfo {
    fn from(val: crate::types::ChatInviteLinkInfo) -> Self {
        Self::ChatInviteLinkInfo(Box::new(val))
    }
}

/// TDLib `ChatJoinRequest` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatJoinRequest {
    /// Describes a user who sent a join request and waits for administrator approval
    #[serde(rename(serialize = "chatJoinRequest", deserialize = "chatJoinRequest"))]
    ChatJoinRequest(Box<crate::types::ChatJoinRequest>),
}

impl ChatJoinRequest {
    /// Convenience constructor to create a [`ChatJoinRequest::ChatJoinRequest`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_join_request(val: crate::types::ChatJoinRequest) -> Self {
        Self::ChatJoinRequest(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatJoinRequest`] into [`ChatJoinRequest`].
impl From<crate::types::ChatJoinRequest> for ChatJoinRequest {
    fn from(val: crate::types::ChatJoinRequest) -> Self {
        Self::ChatJoinRequest(Box::new(val))
    }
}

/// TDLib `ChatJoinRequests` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatJoinRequests {
    /// Contains a list of requests to join a chat
    #[serde(rename(serialize = "chatJoinRequests", deserialize = "chatJoinRequests"))]
    ChatJoinRequests(Box<crate::types::ChatJoinRequests>),
}

impl ChatJoinRequests {
    /// Convenience constructor to create a [`ChatJoinRequests::ChatJoinRequests`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_join_requests(val: crate::types::ChatJoinRequests) -> Self {
        Self::ChatJoinRequests(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatJoinRequests`] into [`ChatJoinRequests`].
impl From<crate::types::ChatJoinRequests> for ChatJoinRequests {
    fn from(val: crate::types::ChatJoinRequests) -> Self {
        Self::ChatJoinRequests(Box::new(val))
    }
}

/// TDLib `ChatJoinRequestsInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatJoinRequestsInfo {
    /// Contains information about pending join requests for a chat
    #[serde(rename(serialize = "chatJoinRequestsInfo", deserialize = "chatJoinRequestsInfo"))]
    ChatJoinRequestsInfo(Box<crate::types::ChatJoinRequestsInfo>),
}

impl ChatJoinRequestsInfo {
    /// Convenience constructor to create a [`ChatJoinRequestsInfo::ChatJoinRequestsInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_join_requests_info(val: crate::types::ChatJoinRequestsInfo) -> Self {
        Self::ChatJoinRequestsInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatJoinRequestsInfo`] into [`ChatJoinRequestsInfo`].
impl From<crate::types::ChatJoinRequestsInfo> for ChatJoinRequestsInfo {
    fn from(val: crate::types::ChatJoinRequestsInfo) -> Self {
        Self::ChatJoinRequestsInfo(Box::new(val))
    }
}

/// TDLib `Supergroup` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Supergroup {
    /// Represents a supergroup or channel with zero or more members (subscribers in the case of channels)
    #[serde(rename(serialize = "supergroup", deserialize = "supergroup"))]
    Supergroup(Box<crate::types::Supergroup>),
}

impl Supergroup {
    /// Convenience constructor to create a [`Supergroup::Supergroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn supergroup(val: crate::types::Supergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }

}

/// Converts a [`crate::types::Supergroup`] into [`Supergroup`].
impl From<crate::types::Supergroup> for Supergroup {
    fn from(val: crate::types::Supergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }
}

/// TDLib `SupergroupFullInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SupergroupFullInfo {
    /// Contains full information about a supergroup or channel
    #[serde(rename(serialize = "supergroupFullInfo", deserialize = "supergroupFullInfo"))]
    SupergroupFullInfo(Box<crate::types::SupergroupFullInfo>),
}

impl SupergroupFullInfo {
    /// Convenience constructor to create a [`SupergroupFullInfo::SupergroupFullInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn supergroup_full_info(val: crate::types::SupergroupFullInfo) -> Self {
        Self::SupergroupFullInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::SupergroupFullInfo`] into [`SupergroupFullInfo`].
impl From<crate::types::SupergroupFullInfo> for SupergroupFullInfo {
    fn from(val: crate::types::SupergroupFullInfo) -> Self {
        Self::SupergroupFullInfo(Box::new(val))
    }
}

/// Describes the current secret chat state
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SecretChatState {
    /// The secret chat is not yet created; waiting for the other user to get online
    #[serde(rename(serialize = "secretChatStatePending", deserialize = "secretChatStatePending"))]
    Pending,
    /// The secret chat is ready to use
    #[serde(rename(serialize = "secretChatStateReady", deserialize = "secretChatStateReady"))]
    Ready,
    /// The secret chat is closed
    #[serde(rename(serialize = "secretChatStateClosed", deserialize = "secretChatStateClosed"))]
    Closed,
}

/// TDLib `SecretChat` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SecretChat {
    /// Represents a secret chat
    #[serde(rename(serialize = "secretChat", deserialize = "secretChat"))]
    SecretChat(Box<crate::types::SecretChat>),
}

impl SecretChat {
    /// Convenience constructor to create a [`SecretChat::SecretChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn secret_chat(val: crate::types::SecretChat) -> Self {
        Self::SecretChat(Box::new(val))
    }

}

/// Converts a [`crate::types::SecretChat`] into [`SecretChat`].
impl From<crate::types::SecretChat> for SecretChat {
    fn from(val: crate::types::SecretChat) -> Self {
        Self::SecretChat(Box::new(val))
    }
}

/// TDLib `SponsoredChat` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SponsoredChat {
    /// Describes a sponsored chat
    #[serde(rename(serialize = "sponsoredChat", deserialize = "sponsoredChat"))]
    SponsoredChat(Box<crate::types::SponsoredChat>),
}

impl SponsoredChat {
    /// Convenience constructor to create a [`SponsoredChat::SponsoredChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sponsored_chat(val: crate::types::SponsoredChat) -> Self {
        Self::SponsoredChat(Box::new(val))
    }

}

/// Converts a [`crate::types::SponsoredChat`] into [`SponsoredChat`].
impl From<crate::types::SponsoredChat> for SponsoredChat {
    fn from(val: crate::types::SponsoredChat) -> Self {
        Self::SponsoredChat(Box::new(val))
    }
}

/// TDLib `SponsoredChats` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SponsoredChats {
    /// Contains a list of sponsored chats
    #[serde(rename(serialize = "sponsoredChats", deserialize = "sponsoredChats"))]
    SponsoredChats(Box<crate::types::SponsoredChats>),
}

impl SponsoredChats {
    /// Convenience constructor to create a [`SponsoredChats::SponsoredChats`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sponsored_chats(val: crate::types::SponsoredChats) -> Self {
        Self::SponsoredChats(Box::new(val))
    }

}

/// Converts a [`crate::types::SponsoredChats`] into [`SponsoredChats`].
impl From<crate::types::SponsoredChats> for SponsoredChats {
    fn from(val: crate::types::SponsoredChats) -> Self {
        Self::SponsoredChats(Box::new(val))
    }
}

/// Describes the type of chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatType {
    /// An ordinary chat with a user
    #[serde(rename(serialize = "chatTypePrivate", deserialize = "chatTypePrivate"))]
    Private(Box<crate::types::ChatTypePrivate>),
    /// A basic group (a chat with 0-200 other users)
    #[serde(rename(serialize = "chatTypeBasicGroup", deserialize = "chatTypeBasicGroup"))]
    BasicGroup(Box<crate::types::ChatTypeBasicGroup>),
    /// A supergroup or channel (with unlimited members)
    #[serde(rename(serialize = "chatTypeSupergroup", deserialize = "chatTypeSupergroup"))]
    Supergroup(Box<crate::types::ChatTypeSupergroup>),
    /// A secret chat with a user
    #[serde(rename(serialize = "chatTypeSecret", deserialize = "chatTypeSecret"))]
    Secret(Box<crate::types::ChatTypeSecret>),
}

impl ChatType {
    /// Convenience constructor to create a [`ChatType::Private`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn private(val: crate::types::ChatTypePrivate) -> Self {
        Self::Private(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatType::BasicGroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn basic_group(val: crate::types::ChatTypeBasicGroup) -> Self {
        Self::BasicGroup(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatType::Supergroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn supergroup(val: crate::types::ChatTypeSupergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatType::Secret`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn secret(val: crate::types::ChatTypeSecret) -> Self {
        Self::Secret(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatTypePrivate`] into [`ChatType`].
impl From<crate::types::ChatTypePrivate> for ChatType {
    fn from(val: crate::types::ChatTypePrivate) -> Self {
        Self::Private(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatTypeBasicGroup`] into [`ChatType`].
impl From<crate::types::ChatTypeBasicGroup> for ChatType {
    fn from(val: crate::types::ChatTypeBasicGroup) -> Self {
        Self::BasicGroup(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatTypeSupergroup`] into [`ChatType`].
impl From<crate::types::ChatTypeSupergroup> for ChatType {
    fn from(val: crate::types::ChatTypeSupergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatTypeSecret`] into [`ChatType`].
impl From<crate::types::ChatTypeSecret> for ChatType {
    fn from(val: crate::types::ChatTypeSecret) -> Self {
        Self::Secret(Box::new(val))
    }
}

/// TDLib `ChatFolderIcon` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderIcon {
    /// Represents an icon for a chat folder
    #[serde(rename(serialize = "chatFolderIcon", deserialize = "chatFolderIcon"))]
    ChatFolderIcon(Box<crate::types::ChatFolderIcon>),
}

impl ChatFolderIcon {
    /// Convenience constructor to create a [`ChatFolderIcon::ChatFolderIcon`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folder_icon(val: crate::types::ChatFolderIcon) -> Self {
        Self::ChatFolderIcon(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatFolderIcon`] into [`ChatFolderIcon`].
impl From<crate::types::ChatFolderIcon> for ChatFolderIcon {
    fn from(val: crate::types::ChatFolderIcon) -> Self {
        Self::ChatFolderIcon(Box::new(val))
    }
}

/// TDLib `ChatFolderName` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderName {
    /// Describes name of a chat folder
    #[serde(rename(serialize = "chatFolderName", deserialize = "chatFolderName"))]
    ChatFolderName(Box<crate::types::ChatFolderName>),
}

impl ChatFolderName {
    /// Convenience constructor to create a [`ChatFolderName::ChatFolderName`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folder_name(val: crate::types::ChatFolderName) -> Self {
        Self::ChatFolderName(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatFolderName`] into [`ChatFolderName`].
impl From<crate::types::ChatFolderName> for ChatFolderName {
    fn from(val: crate::types::ChatFolderName) -> Self {
        Self::ChatFolderName(Box::new(val))
    }
}

/// TDLib `ChatFolder` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolder {
    /// Represents a folder for user chats
    #[serde(rename(serialize = "chatFolder", deserialize = "chatFolder"))]
    ChatFolder(Box<crate::types::ChatFolder>),
}

impl ChatFolder {
    /// Convenience constructor to create a [`ChatFolder::ChatFolder`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folder(val: crate::types::ChatFolder) -> Self {
        Self::ChatFolder(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatFolder`] into [`ChatFolder`].
impl From<crate::types::ChatFolder> for ChatFolder {
    fn from(val: crate::types::ChatFolder) -> Self {
        Self::ChatFolder(Box::new(val))
    }
}

/// TDLib `ChatFolderInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderInfo {
    /// Contains basic information about a chat folder
    #[serde(rename(serialize = "chatFolderInfo", deserialize = "chatFolderInfo"))]
    ChatFolderInfo(Box<crate::types::ChatFolderInfo>),
}

impl ChatFolderInfo {
    /// Convenience constructor to create a [`ChatFolderInfo::ChatFolderInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folder_info(val: crate::types::ChatFolderInfo) -> Self {
        Self::ChatFolderInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatFolderInfo`] into [`ChatFolderInfo`].
impl From<crate::types::ChatFolderInfo> for ChatFolderInfo {
    fn from(val: crate::types::ChatFolderInfo) -> Self {
        Self::ChatFolderInfo(Box::new(val))
    }
}

/// TDLib `ChatFolderInviteLink` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderInviteLink {
    /// Contains a chat folder invite link
    #[serde(rename(serialize = "chatFolderInviteLink", deserialize = "chatFolderInviteLink"))]
    ChatFolderInviteLink(Box<crate::types::ChatFolderInviteLink>),
}

impl ChatFolderInviteLink {
    /// Convenience constructor to create a [`ChatFolderInviteLink::ChatFolderInviteLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folder_invite_link(val: crate::types::ChatFolderInviteLink) -> Self {
        Self::ChatFolderInviteLink(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatFolderInviteLink`] into [`ChatFolderInviteLink`].
impl From<crate::types::ChatFolderInviteLink> for ChatFolderInviteLink {
    fn from(val: crate::types::ChatFolderInviteLink) -> Self {
        Self::ChatFolderInviteLink(Box::new(val))
    }
}

/// TDLib `ChatFolderInviteLinks` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderInviteLinks {
    /// Represents a list of chat folder invite links
    #[serde(rename(serialize = "chatFolderInviteLinks", deserialize = "chatFolderInviteLinks"))]
    ChatFolderInviteLinks(Box<crate::types::ChatFolderInviteLinks>),
}

impl ChatFolderInviteLinks {
    /// Convenience constructor to create a [`ChatFolderInviteLinks::ChatFolderInviteLinks`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folder_invite_links(val: crate::types::ChatFolderInviteLinks) -> Self {
        Self::ChatFolderInviteLinks(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatFolderInviteLinks`] into [`ChatFolderInviteLinks`].
impl From<crate::types::ChatFolderInviteLinks> for ChatFolderInviteLinks {
    fn from(val: crate::types::ChatFolderInviteLinks) -> Self {
        Self::ChatFolderInviteLinks(Box::new(val))
    }
}

/// TDLib `ChatFolderInviteLinkInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderInviteLinkInfo {
    /// Contains information about an invite link to a chat folder
    #[serde(rename(serialize = "chatFolderInviteLinkInfo", deserialize = "chatFolderInviteLinkInfo"))]
    ChatFolderInviteLinkInfo(Box<crate::types::ChatFolderInviteLinkInfo>),
}

impl ChatFolderInviteLinkInfo {
    /// Convenience constructor to create a [`ChatFolderInviteLinkInfo::ChatFolderInviteLinkInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folder_invite_link_info(val: crate::types::ChatFolderInviteLinkInfo) -> Self {
        Self::ChatFolderInviteLinkInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatFolderInviteLinkInfo`] into [`ChatFolderInviteLinkInfo`].
impl From<crate::types::ChatFolderInviteLinkInfo> for ChatFolderInviteLinkInfo {
    fn from(val: crate::types::ChatFolderInviteLinkInfo) -> Self {
        Self::ChatFolderInviteLinkInfo(Box::new(val))
    }
}

/// TDLib `RecommendedChatFolder` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RecommendedChatFolder {
    /// Describes a recommended chat folder
    #[serde(rename(serialize = "recommendedChatFolder", deserialize = "recommendedChatFolder"))]
    RecommendedChatFolder(Box<crate::types::RecommendedChatFolder>),
}

impl RecommendedChatFolder {
    /// Convenience constructor to create a [`RecommendedChatFolder::RecommendedChatFolder`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn recommended_chat_folder(val: crate::types::RecommendedChatFolder) -> Self {
        Self::RecommendedChatFolder(Box::new(val))
    }

}

/// Converts a [`crate::types::RecommendedChatFolder`] into [`RecommendedChatFolder`].
impl From<crate::types::RecommendedChatFolder> for RecommendedChatFolder {
    fn from(val: crate::types::RecommendedChatFolder) -> Self {
        Self::RecommendedChatFolder(Box::new(val))
    }
}

/// TDLib `RecommendedChatFolders` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RecommendedChatFolders {
    /// Contains a list of recommended chat folders
    #[serde(rename(serialize = "recommendedChatFolders", deserialize = "recommendedChatFolders"))]
    RecommendedChatFolders(Box<crate::types::RecommendedChatFolders>),
}

impl RecommendedChatFolders {
    /// Convenience constructor to create a [`RecommendedChatFolders::RecommendedChatFolders`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn recommended_chat_folders(val: crate::types::RecommendedChatFolders) -> Self {
        Self::RecommendedChatFolders(Box::new(val))
    }

}

/// Converts a [`crate::types::RecommendedChatFolders`] into [`RecommendedChatFolders`].
impl From<crate::types::RecommendedChatFolders> for RecommendedChatFolders {
    fn from(val: crate::types::RecommendedChatFolders) -> Self {
        Self::RecommendedChatFolders(Box::new(val))
    }
}

/// TDLib `ArchiveChatListSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ArchiveChatListSettings {
    /// Contains settings for automatic moving of chats to and from the Archive chat lists
    #[serde(rename(serialize = "archiveChatListSettings", deserialize = "archiveChatListSettings"))]
    ArchiveChatListSettings(Box<crate::types::ArchiveChatListSettings>),
}

impl ArchiveChatListSettings {
    /// Convenience constructor to create a [`ArchiveChatListSettings::ArchiveChatListSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn archive_chat_list_settings(val: crate::types::ArchiveChatListSettings) -> Self {
        Self::ArchiveChatListSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::ArchiveChatListSettings`] into [`ArchiveChatListSettings`].
impl From<crate::types::ArchiveChatListSettings> for ArchiveChatListSettings {
    fn from(val: crate::types::ArchiveChatListSettings) -> Self {
        Self::ArchiveChatListSettings(Box::new(val))
    }
}

/// Describes a list of chats
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatList {
    /// A main list of chats
    #[serde(rename(serialize = "chatListMain", deserialize = "chatListMain"))]
    Main,
    /// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
    #[serde(rename(serialize = "chatListArchive", deserialize = "chatListArchive"))]
    Archive,
    /// A list of chats added to a chat folder
    #[serde(rename(serialize = "chatListFolder", deserialize = "chatListFolder"))]
    Folder(Box<crate::types::ChatListFolder>),
}

impl ChatList {
    /// Convenience constructor to create a [`ChatList::Folder`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn folder(val: crate::types::ChatListFolder) -> Self {
        Self::Folder(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatListFolder`] into [`ChatList`].
impl From<crate::types::ChatListFolder> for ChatList {
    fn from(val: crate::types::ChatListFolder) -> Self {
        Self::Folder(Box::new(val))
    }
}

/// TDLib `ChatLists` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatLists {
    /// Contains a list of chat lists
    #[serde(rename(serialize = "chatLists", deserialize = "chatLists"))]
    ChatLists(Box<crate::types::ChatLists>),
}

impl ChatLists {
    /// Convenience constructor to create a [`ChatLists::ChatLists`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_lists(val: crate::types::ChatLists) -> Self {
        Self::ChatLists(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatLists`] into [`ChatLists`].
impl From<crate::types::ChatLists> for ChatLists {
    fn from(val: crate::types::ChatLists) -> Self {
        Self::ChatLists(Box::new(val))
    }
}

/// Describes a reason why an external chat is shown in a chat list
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatSource {
    /// The chat is sponsored by the user's MTProxy server
    #[serde(rename(serialize = "chatSourceMtprotoProxy", deserialize = "chatSourceMtprotoProxy"))]
    MtprotoProxy,
    /// The chat contains a public service announcement
    #[serde(rename(serialize = "chatSourcePublicServiceAnnouncement", deserialize = "chatSourcePublicServiceAnnouncement"))]
    PublicServiceAnnouncement(Box<crate::types::ChatSourcePublicServiceAnnouncement>),
}

impl ChatSource {
    /// Convenience constructor to create a [`ChatSource::PublicServiceAnnouncement`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn public_service_announcement(val: crate::types::ChatSourcePublicServiceAnnouncement) -> Self {
        Self::PublicServiceAnnouncement(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatSourcePublicServiceAnnouncement`] into [`ChatSource`].
impl From<crate::types::ChatSourcePublicServiceAnnouncement> for ChatSource {
    fn from(val: crate::types::ChatSourcePublicServiceAnnouncement) -> Self {
        Self::PublicServiceAnnouncement(Box::new(val))
    }
}

/// TDLib `ChatPosition` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPosition {
    /// Describes a position of a chat in a chat list
    #[serde(rename(serialize = "chatPosition", deserialize = "chatPosition"))]
    ChatPosition(Box<crate::types::ChatPosition>),
}

impl ChatPosition {
    /// Convenience constructor to create a [`ChatPosition::ChatPosition`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_position(val: crate::types::ChatPosition) -> Self {
        Self::ChatPosition(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatPosition`] into [`ChatPosition`].
impl From<crate::types::ChatPosition> for ChatPosition {
    fn from(val: crate::types::ChatPosition) -> Self {
        Self::ChatPosition(Box::new(val))
    }
}

/// TDLib `Chat` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Chat {
    /// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
    #[serde(rename(serialize = "chat", deserialize = "chat"))]
    Chat(Box<crate::types::Chat>),
}

impl Chat {
    /// Convenience constructor to create a [`Chat::Chat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat(val: crate::types::Chat) -> Self {
        Self::Chat(Box::new(val))
    }

}

/// Converts a [`crate::types::Chat`] into [`Chat`].
impl From<crate::types::Chat> for Chat {
    fn from(val: crate::types::Chat) -> Self {
        Self::Chat(Box::new(val))
    }
}

/// TDLib `Chats` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Chats {
    /// Represents a list of chats
    #[serde(rename(serialize = "chats", deserialize = "chats"))]
    Chats(Box<crate::types::Chats>),
}

impl Chats {
    /// Convenience constructor to create a [`Chats::Chats`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chats(val: crate::types::Chats) -> Self {
        Self::Chats(Box::new(val))
    }

}

/// Converts a [`crate::types::Chats`] into [`Chats`].
impl From<crate::types::Chats> for Chats {
    fn from(val: crate::types::Chats) -> Self {
        Self::Chats(Box::new(val))
    }
}

/// TDLib `CreatedBasicGroupChat` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CreatedBasicGroupChat {
    /// Contains information about a newly created basic group chat
    #[serde(rename(serialize = "createdBasicGroupChat", deserialize = "createdBasicGroupChat"))]
    CreatedBasicGroupChat(Box<crate::types::CreatedBasicGroupChat>),
}

impl CreatedBasicGroupChat {
    /// Convenience constructor to create a [`CreatedBasicGroupChat::CreatedBasicGroupChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn created_basic_group_chat(val: crate::types::CreatedBasicGroupChat) -> Self {
        Self::CreatedBasicGroupChat(Box::new(val))
    }

}

/// Converts a [`crate::types::CreatedBasicGroupChat`] into [`CreatedBasicGroupChat`].
impl From<crate::types::CreatedBasicGroupChat> for CreatedBasicGroupChat {
    fn from(val: crate::types::CreatedBasicGroupChat) -> Self {
        Self::CreatedBasicGroupChat(Box::new(val))
    }
}

/// Describes type of public chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicChatType {
    /// The chat is public, because it has an active username
    #[serde(rename(serialize = "publicChatTypeHasUsername", deserialize = "publicChatTypeHasUsername"))]
    HasUsername,
    /// The chat is public, because it is a location-based supergroup
    #[serde(rename(serialize = "publicChatTypeIsLocationBased", deserialize = "publicChatTypeIsLocationBased"))]
    IsLocationBased,
}

/// Describes actions which must be possible to do through a chat action bar
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatActionBar {
    /// The chat can be reported as spam using the method reportChat with an empty option_id and message_ids. If the chat is a private chat with a user with an emoji status, then a notice about emoji status usage must be shown
    #[serde(rename(serialize = "chatActionBarReportSpam", deserialize = "chatActionBarReportSpam"))]
    ReportSpam(Box<crate::types::ChatActionBarReportSpam>),
    /// The chat is a recently created group chat to which new members can be invited
    #[serde(rename(serialize = "chatActionBarInviteMembers", deserialize = "chatActionBarInviteMembers"))]
    InviteMembers,
    /// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method setMessageSenderBlockList,
    /// or the other user can be added to the contact list using the method addContact. If the chat is a private chat with a user with an emoji status, then a notice about emoji status usage must be shown
    #[serde(rename(serialize = "chatActionBarReportAddBlock", deserialize = "chatActionBarReportAddBlock"))]
    ReportAddBlock(Box<crate::types::ChatActionBarReportAddBlock>),
    /// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
    #[serde(rename(serialize = "chatActionBarAddContact", deserialize = "chatActionBarAddContact"))]
    AddContact,
    /// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
    #[serde(rename(serialize = "chatActionBarSharePhoneNumber", deserialize = "chatActionBarSharePhoneNumber"))]
    SharePhoneNumber,
    /// The chat is a private chat with an administrator of a chat to which the user sent join request
    #[serde(rename(serialize = "chatActionBarJoinRequest", deserialize = "chatActionBarJoinRequest"))]
    JoinRequest(Box<crate::types::ChatActionBarJoinRequest>),
}

impl ChatActionBar {
    /// Convenience constructor to create a [`ChatActionBar::ReportSpam`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn report_spam(val: crate::types::ChatActionBarReportSpam) -> Self {
        Self::ReportSpam(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatActionBar::ReportAddBlock`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn report_add_block(val: crate::types::ChatActionBarReportAddBlock) -> Self {
        Self::ReportAddBlock(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatActionBar::JoinRequest`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn join_request(val: crate::types::ChatActionBarJoinRequest) -> Self {
        Self::JoinRequest(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatActionBarReportSpam`] into [`ChatActionBar`].
impl From<crate::types::ChatActionBarReportSpam> for ChatActionBar {
    fn from(val: crate::types::ChatActionBarReportSpam) -> Self {
        Self::ReportSpam(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatActionBarReportAddBlock`] into [`ChatActionBar`].
impl From<crate::types::ChatActionBarReportAddBlock> for ChatActionBar {
    fn from(val: crate::types::ChatActionBarReportAddBlock) -> Self {
        Self::ReportAddBlock(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatActionBarJoinRequest`] into [`ChatActionBar`].
impl From<crate::types::ChatActionBarJoinRequest> for ChatActionBar {
    fn from(val: crate::types::ChatActionBarJoinRequest) -> Self {
        Self::JoinRequest(Box::new(val))
    }
}

/// TDLib `SharedChat` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SharedChat {
    /// Contains information about a chat shared with a bot
    #[serde(rename(serialize = "sharedChat", deserialize = "sharedChat"))]
    SharedChat(Box<crate::types::SharedChat>),
}

impl SharedChat {
    /// Convenience constructor to create a [`SharedChat::SharedChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn shared_chat(val: crate::types::SharedChat) -> Self {
        Self::SharedChat(Box::new(val))
    }

}

/// Converts a [`crate::types::SharedChat`] into [`SharedChat`].
impl From<crate::types::SharedChat> for SharedChat {
    fn from(val: crate::types::SharedChat) -> Self {
        Self::SharedChat(Box::new(val))
    }
}

/// Represents a filter for type of the chats to search for
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SearchChatTypeFilter {
    /// Returns only private chats with bots
    #[serde(rename(serialize = "searchChatTypeFilterBot", deserialize = "searchChatTypeFilterBot"))]
    Bot,
    /// Returns only channel chats
    #[serde(rename(serialize = "searchChatTypeFilterChannel", deserialize = "searchChatTypeFilterChannel"))]
    Channel,
}

/// Describes the different types of activity in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAction {
    /// The user is typing a message
    #[serde(rename(serialize = "chatActionTyping", deserialize = "chatActionTyping"))]
    Typing,
    /// The user is recording a video
    #[serde(rename(serialize = "chatActionRecordingVideo", deserialize = "chatActionRecordingVideo"))]
    RecordingVideo,
    /// The user is uploading a video
    #[serde(rename(serialize = "chatActionUploadingVideo", deserialize = "chatActionUploadingVideo"))]
    UploadingVideo(Box<crate::types::ChatActionUploadingVideo>),
    /// The user is recording a voice note
    #[serde(rename(serialize = "chatActionRecordingVoiceNote", deserialize = "chatActionRecordingVoiceNote"))]
    RecordingVoiceNote,
    /// The user is uploading a voice note
    #[serde(rename(serialize = "chatActionUploadingVoiceNote", deserialize = "chatActionUploadingVoiceNote"))]
    UploadingVoiceNote(Box<crate::types::ChatActionUploadingVoiceNote>),
    /// The user is uploading a photo
    #[serde(rename(serialize = "chatActionUploadingPhoto", deserialize = "chatActionUploadingPhoto"))]
    UploadingPhoto(Box<crate::types::ChatActionUploadingPhoto>),
    /// The user is uploading a document
    #[serde(rename(serialize = "chatActionUploadingDocument", deserialize = "chatActionUploadingDocument"))]
    UploadingDocument(Box<crate::types::ChatActionUploadingDocument>),
    /// The user is picking a sticker to send
    #[serde(rename(serialize = "chatActionChoosingSticker", deserialize = "chatActionChoosingSticker"))]
    ChoosingSticker,
    /// The user is picking a location or venue to send
    #[serde(rename(serialize = "chatActionChoosingLocation", deserialize = "chatActionChoosingLocation"))]
    ChoosingLocation,
    /// The user is picking a contact to send
    #[serde(rename(serialize = "chatActionChoosingContact", deserialize = "chatActionChoosingContact"))]
    ChoosingContact,
    /// The user has started to play a game
    #[serde(rename(serialize = "chatActionStartPlayingGame", deserialize = "chatActionStartPlayingGame"))]
    StartPlayingGame,
    /// The user is recording a video note
    #[serde(rename(serialize = "chatActionRecordingVideoNote", deserialize = "chatActionRecordingVideoNote"))]
    RecordingVideoNote,
    /// The user is uploading a video note
    #[serde(rename(serialize = "chatActionUploadingVideoNote", deserialize = "chatActionUploadingVideoNote"))]
    UploadingVideoNote(Box<crate::types::ChatActionUploadingVideoNote>),
    /// The user is watching animations sent by the other party by clicking on an animated emoji
    #[serde(rename(serialize = "chatActionWatchingAnimations", deserialize = "chatActionWatchingAnimations"))]
    WatchingAnimations(Box<crate::types::ChatActionWatchingAnimations>),
    /// The user has canceled the previous action
    #[serde(rename(serialize = "chatActionCancel", deserialize = "chatActionCancel"))]
    Cancel,
}

impl ChatAction {
    /// Convenience constructor to create a [`ChatAction::UploadingVideo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn uploading_video(val: crate::types::ChatActionUploadingVideo) -> Self {
        Self::UploadingVideo(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatAction::UploadingVoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn uploading_voice_note(val: crate::types::ChatActionUploadingVoiceNote) -> Self {
        Self::UploadingVoiceNote(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatAction::UploadingPhoto`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn uploading_photo(val: crate::types::ChatActionUploadingPhoto) -> Self {
        Self::UploadingPhoto(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatAction::UploadingDocument`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn uploading_document(val: crate::types::ChatActionUploadingDocument) -> Self {
        Self::UploadingDocument(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatAction::UploadingVideoNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn uploading_video_note(val: crate::types::ChatActionUploadingVideoNote) -> Self {
        Self::UploadingVideoNote(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatAction::WatchingAnimations`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn watching_animations(val: crate::types::ChatActionWatchingAnimations) -> Self {
        Self::WatchingAnimations(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatActionUploadingVideo`] into [`ChatAction`].
impl From<crate::types::ChatActionUploadingVideo> for ChatAction {
    fn from(val: crate::types::ChatActionUploadingVideo) -> Self {
        Self::UploadingVideo(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatActionUploadingVoiceNote`] into [`ChatAction`].
impl From<crate::types::ChatActionUploadingVoiceNote> for ChatAction {
    fn from(val: crate::types::ChatActionUploadingVoiceNote) -> Self {
        Self::UploadingVoiceNote(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatActionUploadingPhoto`] into [`ChatAction`].
impl From<crate::types::ChatActionUploadingPhoto> for ChatAction {
    fn from(val: crate::types::ChatActionUploadingPhoto) -> Self {
        Self::UploadingPhoto(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatActionUploadingDocument`] into [`ChatAction`].
impl From<crate::types::ChatActionUploadingDocument> for ChatAction {
    fn from(val: crate::types::ChatActionUploadingDocument) -> Self {
        Self::UploadingDocument(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatActionUploadingVideoNote`] into [`ChatAction`].
impl From<crate::types::ChatActionUploadingVideoNote> for ChatAction {
    fn from(val: crate::types::ChatActionUploadingVideoNote) -> Self {
        Self::UploadingVideoNote(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatActionWatchingAnimations`] into [`ChatAction`].
impl From<crate::types::ChatActionWatchingAnimations> for ChatAction {
    fn from(val: crate::types::ChatActionWatchingAnimations) -> Self {
        Self::WatchingAnimations(Box::new(val))
    }
}

/// TDLib `TargetChatTypes` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TargetChatTypes {
    /// Describes allowed types for the target chat
    #[serde(rename(serialize = "targetChatTypes", deserialize = "targetChatTypes"))]
    TargetChatTypes(Box<crate::types::TargetChatTypes>),
}

impl TargetChatTypes {
    /// Convenience constructor to create a [`TargetChatTypes::TargetChatTypes`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn target_chat_types(val: crate::types::TargetChatTypes) -> Self {
        Self::TargetChatTypes(Box::new(val))
    }

}

/// Converts a [`crate::types::TargetChatTypes`] into [`TargetChatTypes`].
impl From<crate::types::TargetChatTypes> for TargetChatTypes {
    fn from(val: crate::types::TargetChatTypes) -> Self {
        Self::TargetChatTypes(Box::new(val))
    }
}

/// Describes the target chat to be opened
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TargetChat {
    /// The currently opened chat and forum topic must be kept
    #[serde(rename(serialize = "targetChatCurrent", deserialize = "targetChatCurrent"))]
    Current,
    /// The chat needs to be chosen by the user among chats of the specified types
    #[serde(rename(serialize = "targetChatChosen", deserialize = "targetChatChosen"))]
    Chosen(Box<crate::types::TargetChatChosen>),
    /// The chat needs to be open with the provided internal link
    #[serde(rename(serialize = "targetChatInternalLink", deserialize = "targetChatInternalLink"))]
    InternalLink(Box<crate::types::TargetChatInternalLink>),
}

impl TargetChat {
    /// Convenience constructor to create a [`TargetChat::Chosen`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chosen(val: crate::types::TargetChatChosen) -> Self {
        Self::Chosen(Box::new(val))
    }

    /// Convenience constructor to create a [`TargetChat::InternalLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn internal_link(val: crate::types::TargetChatInternalLink) -> Self {
        Self::InternalLink(Box::new(val))
    }

}

/// Converts a [`crate::types::TargetChatChosen`] into [`TargetChat`].
impl From<crate::types::TargetChatChosen> for TargetChat {
    fn from(val: crate::types::TargetChatChosen) -> Self {
        Self::Chosen(Box::new(val))
    }
}

/// Converts a [`crate::types::TargetChatInternalLink`] into [`TargetChat`].
impl From<crate::types::TargetChatInternalLink> for TargetChat {
    fn from(val: crate::types::TargetChatInternalLink) -> Self {
        Self::InternalLink(Box::new(val))
    }
}

/// Represents a chat event
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEventAction {
    /// A message was edited
    #[serde(rename(serialize = "chatEventMessageEdited", deserialize = "chatEventMessageEdited"))]
    ChatEventMessageEdited(Box<crate::types::ChatEventMessageEdited>),
    /// A message was deleted
    #[serde(rename(serialize = "chatEventMessageDeleted", deserialize = "chatEventMessageDeleted"))]
    ChatEventMessageDeleted(Box<crate::types::ChatEventMessageDeleted>),
    /// A message was pinned
    #[serde(rename(serialize = "chatEventMessagePinned", deserialize = "chatEventMessagePinned"))]
    ChatEventMessagePinned(Box<crate::types::ChatEventMessagePinned>),
    /// A message was unpinned
    #[serde(rename(serialize = "chatEventMessageUnpinned", deserialize = "chatEventMessageUnpinned"))]
    ChatEventMessageUnpinned(Box<crate::types::ChatEventMessageUnpinned>),
    /// A poll in a message was stopped
    #[serde(rename(serialize = "chatEventPollStopped", deserialize = "chatEventPollStopped"))]
    ChatEventPollStopped(Box<crate::types::ChatEventPollStopped>),
    /// A new member joined the chat
    #[serde(rename(serialize = "chatEventMemberJoined", deserialize = "chatEventMemberJoined"))]
    ChatEventMemberJoined,
    /// A new member joined the chat via an invite link
    #[serde(rename(serialize = "chatEventMemberJoinedByInviteLink", deserialize = "chatEventMemberJoinedByInviteLink"))]
    ChatEventMemberJoinedByInviteLink(Box<crate::types::ChatEventMemberJoinedByInviteLink>),
    /// A new member was accepted to the chat by an administrator
    #[serde(rename(serialize = "chatEventMemberJoinedByRequest", deserialize = "chatEventMemberJoinedByRequest"))]
    ChatEventMemberJoinedByRequest(Box<crate::types::ChatEventMemberJoinedByRequest>),
    /// A new chat member was invited
    #[serde(rename(serialize = "chatEventMemberInvited", deserialize = "chatEventMemberInvited"))]
    ChatEventMemberInvited(Box<crate::types::ChatEventMemberInvited>),
    /// A member left the chat
    #[serde(rename(serialize = "chatEventMemberLeft", deserialize = "chatEventMemberLeft"))]
    ChatEventMemberLeft,
    /// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
    #[serde(rename(serialize = "chatEventMemberPromoted", deserialize = "chatEventMemberPromoted"))]
    ChatEventMemberPromoted(Box<crate::types::ChatEventMemberPromoted>),
    /// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
    #[serde(rename(serialize = "chatEventMemberRestricted", deserialize = "chatEventMemberRestricted"))]
    ChatEventMemberRestricted(Box<crate::types::ChatEventMemberRestricted>),
    /// A chat member tag has been changed
    #[serde(rename(serialize = "chatEventMemberTagChanged", deserialize = "chatEventMemberTagChanged"))]
    ChatEventMemberTagChanged(Box<crate::types::ChatEventMemberTagChanged>),
    /// A chat member extended their subscription to the chat
    #[serde(rename(serialize = "chatEventMemberSubscriptionExtended", deserialize = "chatEventMemberSubscriptionExtended"))]
    ChatEventMemberSubscriptionExtended(Box<crate::types::ChatEventMemberSubscriptionExtended>),
    /// The chat available reactions were changed
    #[serde(rename(serialize = "chatEventAvailableReactionsChanged", deserialize = "chatEventAvailableReactionsChanged"))]
    ChatEventAvailableReactionsChanged(Box<crate::types::ChatEventAvailableReactionsChanged>),
    /// The chat background was changed
    #[serde(rename(serialize = "chatEventBackgroundChanged", deserialize = "chatEventBackgroundChanged"))]
    ChatEventBackgroundChanged(Box<crate::types::ChatEventBackgroundChanged>),
    /// The chat description was changed
    #[serde(rename(serialize = "chatEventDescriptionChanged", deserialize = "chatEventDescriptionChanged"))]
    ChatEventDescriptionChanged(Box<crate::types::ChatEventDescriptionChanged>),
    /// The chat emoji status was changed
    #[serde(rename(serialize = "chatEventEmojiStatusChanged", deserialize = "chatEventEmojiStatusChanged"))]
    ChatEventEmojiStatusChanged(Box<crate::types::ChatEventEmojiStatusChanged>),
    /// The linked chat of a supergroup was changed
    #[serde(rename(serialize = "chatEventLinkedChatChanged", deserialize = "chatEventLinkedChatChanged"))]
    ChatEventLinkedChatChanged(Box<crate::types::ChatEventLinkedChatChanged>),
    /// The supergroup location was changed
    #[serde(rename(serialize = "chatEventLocationChanged", deserialize = "chatEventLocationChanged"))]
    ChatEventLocationChanged(Box<crate::types::ChatEventLocationChanged>),
    /// The message auto-delete timer was changed
    #[serde(rename(serialize = "chatEventMessageAutoDeleteTimeChanged", deserialize = "chatEventMessageAutoDeleteTimeChanged"))]
    ChatEventMessageAutoDeleteTimeChanged(Box<crate::types::ChatEventMessageAutoDeleteTimeChanged>),
    /// The chat permissions were changed
    #[serde(rename(serialize = "chatEventPermissionsChanged", deserialize = "chatEventPermissionsChanged"))]
    ChatEventPermissionsChanged(Box<crate::types::ChatEventPermissionsChanged>),
    /// The chat photo was changed
    #[serde(rename(serialize = "chatEventPhotoChanged", deserialize = "chatEventPhotoChanged"))]
    ChatEventPhotoChanged(Box<crate::types::ChatEventPhotoChanged>),
    /// The slow_mode_delay setting of a supergroup was changed
    #[serde(rename(serialize = "chatEventSlowModeDelayChanged", deserialize = "chatEventSlowModeDelayChanged"))]
    ChatEventSlowModeDelayChanged(Box<crate::types::ChatEventSlowModeDelayChanged>),
    /// The supergroup sticker set was changed
    #[serde(rename(serialize = "chatEventStickerSetChanged", deserialize = "chatEventStickerSetChanged"))]
    ChatEventStickerSetChanged(Box<crate::types::ChatEventStickerSetChanged>),
    /// The supergroup sticker set with allowed custom emoji was changed
    #[serde(rename(serialize = "chatEventCustomEmojiStickerSetChanged", deserialize = "chatEventCustomEmojiStickerSetChanged"))]
    ChatEventCustomEmojiStickerSetChanged(Box<crate::types::ChatEventCustomEmojiStickerSetChanged>),
    /// The chat title was changed
    #[serde(rename(serialize = "chatEventTitleChanged", deserialize = "chatEventTitleChanged"))]
    ChatEventTitleChanged(Box<crate::types::ChatEventTitleChanged>),
    /// The chat editable username was changed
    #[serde(rename(serialize = "chatEventUsernameChanged", deserialize = "chatEventUsernameChanged"))]
    ChatEventUsernameChanged(Box<crate::types::ChatEventUsernameChanged>),
    /// The chat active usernames were changed
    #[serde(rename(serialize = "chatEventActiveUsernamesChanged", deserialize = "chatEventActiveUsernamesChanged"))]
    ChatEventActiveUsernamesChanged(Box<crate::types::ChatEventActiveUsernamesChanged>),
    /// The chat accent color or background custom emoji were changed
    #[serde(rename(serialize = "chatEventAccentColorChanged", deserialize = "chatEventAccentColorChanged"))]
    ChatEventAccentColorChanged(Box<crate::types::ChatEventAccentColorChanged>),
    /// The chat's profile accent color or profile background custom emoji were changed
    #[serde(rename(serialize = "chatEventProfileAccentColorChanged", deserialize = "chatEventProfileAccentColorChanged"))]
    ChatEventProfileAccentColorChanged(Box<crate::types::ChatEventProfileAccentColorChanged>),
    /// The has_protected_content setting of a chat was toggled
    #[serde(rename(serialize = "chatEventHasProtectedContentToggled", deserialize = "chatEventHasProtectedContentToggled"))]
    ChatEventHasProtectedContentToggled(Box<crate::types::ChatEventHasProtectedContentToggled>),
    /// The can_invite_users permission of a supergroup chat was toggled
    #[serde(rename(serialize = "chatEventInvitesToggled", deserialize = "chatEventInvitesToggled"))]
    ChatEventInvitesToggled(Box<crate::types::ChatEventInvitesToggled>),
    /// The is_all_history_available setting of a supergroup was toggled
    #[serde(rename(serialize = "chatEventIsAllHistoryAvailableToggled", deserialize = "chatEventIsAllHistoryAvailableToggled"))]
    ChatEventIsAllHistoryAvailableToggled(Box<crate::types::ChatEventIsAllHistoryAvailableToggled>),
    /// The has_aggressive_anti_spam_enabled setting of a supergroup was toggled
    #[serde(rename(serialize = "chatEventHasAggressiveAntiSpamEnabledToggled", deserialize = "chatEventHasAggressiveAntiSpamEnabledToggled"))]
    ChatEventHasAggressiveAntiSpamEnabledToggled(Box<crate::types::ChatEventHasAggressiveAntiSpamEnabledToggled>),
    /// The sign_messages setting of a channel was toggled
    #[serde(rename(serialize = "chatEventSignMessagesToggled", deserialize = "chatEventSignMessagesToggled"))]
    ChatEventSignMessagesToggled(Box<crate::types::ChatEventSignMessagesToggled>),
    /// The show_message_sender setting of a channel was toggled
    #[serde(rename(serialize = "chatEventShowMessageSenderToggled", deserialize = "chatEventShowMessageSenderToggled"))]
    ChatEventShowMessageSenderToggled(Box<crate::types::ChatEventShowMessageSenderToggled>),
    /// The has_automatic_translation setting of a channel was toggled
    #[serde(rename(serialize = "chatEventAutomaticTranslationToggled", deserialize = "chatEventAutomaticTranslationToggled"))]
    ChatEventAutomaticTranslationToggled(Box<crate::types::ChatEventAutomaticTranslationToggled>),
    /// A chat invite link was edited
    #[serde(rename(serialize = "chatEventInviteLinkEdited", deserialize = "chatEventInviteLinkEdited"))]
    ChatEventInviteLinkEdited(Box<crate::types::ChatEventInviteLinkEdited>),
    /// A chat invite link was revoked
    #[serde(rename(serialize = "chatEventInviteLinkRevoked", deserialize = "chatEventInviteLinkRevoked"))]
    ChatEventInviteLinkRevoked(Box<crate::types::ChatEventInviteLinkRevoked>),
    /// A revoked chat invite link was deleted
    #[serde(rename(serialize = "chatEventInviteLinkDeleted", deserialize = "chatEventInviteLinkDeleted"))]
    ChatEventInviteLinkDeleted(Box<crate::types::ChatEventInviteLinkDeleted>),
    /// A video chat was created
    #[serde(rename(serialize = "chatEventVideoChatCreated", deserialize = "chatEventVideoChatCreated"))]
    ChatEventVideoChatCreated(Box<crate::types::ChatEventVideoChatCreated>),
    /// A video chat was ended
    #[serde(rename(serialize = "chatEventVideoChatEnded", deserialize = "chatEventVideoChatEnded"))]
    ChatEventVideoChatEnded(Box<crate::types::ChatEventVideoChatEnded>),
    /// The mute_new_participants setting of a video chat was toggled
    #[serde(rename(serialize = "chatEventVideoChatMuteNewParticipantsToggled", deserialize = "chatEventVideoChatMuteNewParticipantsToggled"))]
    ChatEventVideoChatMuteNewParticipantsToggled(Box<crate::types::ChatEventVideoChatMuteNewParticipantsToggled>),
    /// A video chat participant was muted or unmuted
    #[serde(rename(serialize = "chatEventVideoChatParticipantIsMutedToggled", deserialize = "chatEventVideoChatParticipantIsMutedToggled"))]
    ChatEventVideoChatParticipantIsMutedToggled(Box<crate::types::ChatEventVideoChatParticipantIsMutedToggled>),
    /// A video chat participant volume level was changed
    #[serde(rename(serialize = "chatEventVideoChatParticipantVolumeLevelChanged", deserialize = "chatEventVideoChatParticipantVolumeLevelChanged"))]
    ChatEventVideoChatParticipantVolumeLevelChanged(Box<crate::types::ChatEventVideoChatParticipantVolumeLevelChanged>),
    /// The is_forum setting of a supergroup was toggled
    #[serde(rename(serialize = "chatEventIsForumToggled", deserialize = "chatEventIsForumToggled"))]
    ChatEventIsForumToggled(Box<crate::types::ChatEventIsForumToggled>),
    /// A new forum topic was created
    #[serde(rename(serialize = "chatEventForumTopicCreated", deserialize = "chatEventForumTopicCreated"))]
    ChatEventForumTopicCreated(Box<crate::types::ChatEventForumTopicCreated>),
    /// A forum topic was edited
    #[serde(rename(serialize = "chatEventForumTopicEdited", deserialize = "chatEventForumTopicEdited"))]
    ChatEventForumTopicEdited(Box<crate::types::ChatEventForumTopicEdited>),
    /// A forum topic was closed or reopened
    #[serde(rename(serialize = "chatEventForumTopicToggleIsClosed", deserialize = "chatEventForumTopicToggleIsClosed"))]
    ChatEventForumTopicToggleIsClosed(Box<crate::types::ChatEventForumTopicToggleIsClosed>),
    /// The General forum topic was hidden or unhidden
    #[serde(rename(serialize = "chatEventForumTopicToggleIsHidden", deserialize = "chatEventForumTopicToggleIsHidden"))]
    ChatEventForumTopicToggleIsHidden(Box<crate::types::ChatEventForumTopicToggleIsHidden>),
    /// A forum topic was deleted
    #[serde(rename(serialize = "chatEventForumTopicDeleted", deserialize = "chatEventForumTopicDeleted"))]
    ChatEventForumTopicDeleted(Box<crate::types::ChatEventForumTopicDeleted>),
    /// A pinned forum topic was changed
    #[serde(rename(serialize = "chatEventForumTopicPinned", deserialize = "chatEventForumTopicPinned"))]
    ChatEventForumTopicPinned(Box<crate::types::ChatEventForumTopicPinned>),
}

impl ChatEventAction {
    /// Convenience constructor to create a [`ChatEventAction::ChatEventMessageEdited`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_message_edited(val: crate::types::ChatEventMessageEdited) -> Self {
        Self::ChatEventMessageEdited(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMessageDeleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_message_deleted(val: crate::types::ChatEventMessageDeleted) -> Self {
        Self::ChatEventMessageDeleted(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMessagePinned`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_message_pinned(val: crate::types::ChatEventMessagePinned) -> Self {
        Self::ChatEventMessagePinned(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMessageUnpinned`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_message_unpinned(val: crate::types::ChatEventMessageUnpinned) -> Self {
        Self::ChatEventMessageUnpinned(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventPollStopped`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_poll_stopped(val: crate::types::ChatEventPollStopped) -> Self {
        Self::ChatEventPollStopped(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMemberJoinedByInviteLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_member_joined_by_invite_link(val: crate::types::ChatEventMemberJoinedByInviteLink) -> Self {
        Self::ChatEventMemberJoinedByInviteLink(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMemberJoinedByRequest`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_member_joined_by_request(val: crate::types::ChatEventMemberJoinedByRequest) -> Self {
        Self::ChatEventMemberJoinedByRequest(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMemberInvited`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_member_invited(val: crate::types::ChatEventMemberInvited) -> Self {
        Self::ChatEventMemberInvited(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMemberPromoted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_member_promoted(val: crate::types::ChatEventMemberPromoted) -> Self {
        Self::ChatEventMemberPromoted(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMemberRestricted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_member_restricted(val: crate::types::ChatEventMemberRestricted) -> Self {
        Self::ChatEventMemberRestricted(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMemberTagChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_member_tag_changed(val: crate::types::ChatEventMemberTagChanged) -> Self {
        Self::ChatEventMemberTagChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMemberSubscriptionExtended`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_member_subscription_extended(val: crate::types::ChatEventMemberSubscriptionExtended) -> Self {
        Self::ChatEventMemberSubscriptionExtended(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventAvailableReactionsChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_available_reactions_changed(val: crate::types::ChatEventAvailableReactionsChanged) -> Self {
        Self::ChatEventAvailableReactionsChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventBackgroundChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_background_changed(val: crate::types::ChatEventBackgroundChanged) -> Self {
        Self::ChatEventBackgroundChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventDescriptionChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_description_changed(val: crate::types::ChatEventDescriptionChanged) -> Self {
        Self::ChatEventDescriptionChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventEmojiStatusChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_emoji_status_changed(val: crate::types::ChatEventEmojiStatusChanged) -> Self {
        Self::ChatEventEmojiStatusChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventLinkedChatChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_linked_chat_changed(val: crate::types::ChatEventLinkedChatChanged) -> Self {
        Self::ChatEventLinkedChatChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventLocationChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_location_changed(val: crate::types::ChatEventLocationChanged) -> Self {
        Self::ChatEventLocationChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventMessageAutoDeleteTimeChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_message_auto_delete_time_changed(val: crate::types::ChatEventMessageAutoDeleteTimeChanged) -> Self {
        Self::ChatEventMessageAutoDeleteTimeChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventPermissionsChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_permissions_changed(val: crate::types::ChatEventPermissionsChanged) -> Self {
        Self::ChatEventPermissionsChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventPhotoChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_photo_changed(val: crate::types::ChatEventPhotoChanged) -> Self {
        Self::ChatEventPhotoChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventSlowModeDelayChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_slow_mode_delay_changed(val: crate::types::ChatEventSlowModeDelayChanged) -> Self {
        Self::ChatEventSlowModeDelayChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventStickerSetChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_sticker_set_changed(val: crate::types::ChatEventStickerSetChanged) -> Self {
        Self::ChatEventStickerSetChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventCustomEmojiStickerSetChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_custom_emoji_sticker_set_changed(val: crate::types::ChatEventCustomEmojiStickerSetChanged) -> Self {
        Self::ChatEventCustomEmojiStickerSetChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventTitleChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_title_changed(val: crate::types::ChatEventTitleChanged) -> Self {
        Self::ChatEventTitleChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventUsernameChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_username_changed(val: crate::types::ChatEventUsernameChanged) -> Self {
        Self::ChatEventUsernameChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventActiveUsernamesChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_active_usernames_changed(val: crate::types::ChatEventActiveUsernamesChanged) -> Self {
        Self::ChatEventActiveUsernamesChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventAccentColorChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_accent_color_changed(val: crate::types::ChatEventAccentColorChanged) -> Self {
        Self::ChatEventAccentColorChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventProfileAccentColorChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_profile_accent_color_changed(val: crate::types::ChatEventProfileAccentColorChanged) -> Self {
        Self::ChatEventProfileAccentColorChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventHasProtectedContentToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_has_protected_content_toggled(val: crate::types::ChatEventHasProtectedContentToggled) -> Self {
        Self::ChatEventHasProtectedContentToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventInvitesToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_invites_toggled(val: crate::types::ChatEventInvitesToggled) -> Self {
        Self::ChatEventInvitesToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventIsAllHistoryAvailableToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_is_all_history_available_toggled(val: crate::types::ChatEventIsAllHistoryAvailableToggled) -> Self {
        Self::ChatEventIsAllHistoryAvailableToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventHasAggressiveAntiSpamEnabledToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_has_aggressive_anti_spam_enabled_toggled(val: crate::types::ChatEventHasAggressiveAntiSpamEnabledToggled) -> Self {
        Self::ChatEventHasAggressiveAntiSpamEnabledToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventSignMessagesToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_sign_messages_toggled(val: crate::types::ChatEventSignMessagesToggled) -> Self {
        Self::ChatEventSignMessagesToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventShowMessageSenderToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_show_message_sender_toggled(val: crate::types::ChatEventShowMessageSenderToggled) -> Self {
        Self::ChatEventShowMessageSenderToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventAutomaticTranslationToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_automatic_translation_toggled(val: crate::types::ChatEventAutomaticTranslationToggled) -> Self {
        Self::ChatEventAutomaticTranslationToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventInviteLinkEdited`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_invite_link_edited(val: crate::types::ChatEventInviteLinkEdited) -> Self {
        Self::ChatEventInviteLinkEdited(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventInviteLinkRevoked`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_invite_link_revoked(val: crate::types::ChatEventInviteLinkRevoked) -> Self {
        Self::ChatEventInviteLinkRevoked(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventInviteLinkDeleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_invite_link_deleted(val: crate::types::ChatEventInviteLinkDeleted) -> Self {
        Self::ChatEventInviteLinkDeleted(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventVideoChatCreated`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_video_chat_created(val: crate::types::ChatEventVideoChatCreated) -> Self {
        Self::ChatEventVideoChatCreated(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventVideoChatEnded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_video_chat_ended(val: crate::types::ChatEventVideoChatEnded) -> Self {
        Self::ChatEventVideoChatEnded(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventVideoChatMuteNewParticipantsToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_video_chat_mute_new_participants_toggled(val: crate::types::ChatEventVideoChatMuteNewParticipantsToggled) -> Self {
        Self::ChatEventVideoChatMuteNewParticipantsToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventVideoChatParticipantIsMutedToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_video_chat_participant_is_muted_toggled(val: crate::types::ChatEventVideoChatParticipantIsMutedToggled) -> Self {
        Self::ChatEventVideoChatParticipantIsMutedToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventVideoChatParticipantVolumeLevelChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_video_chat_participant_volume_level_changed(val: crate::types::ChatEventVideoChatParticipantVolumeLevelChanged) -> Self {
        Self::ChatEventVideoChatParticipantVolumeLevelChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventIsForumToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_is_forum_toggled(val: crate::types::ChatEventIsForumToggled) -> Self {
        Self::ChatEventIsForumToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventForumTopicCreated`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_forum_topic_created(val: crate::types::ChatEventForumTopicCreated) -> Self {
        Self::ChatEventForumTopicCreated(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventForumTopicEdited`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_forum_topic_edited(val: crate::types::ChatEventForumTopicEdited) -> Self {
        Self::ChatEventForumTopicEdited(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventForumTopicToggleIsClosed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_forum_topic_toggle_is_closed(val: crate::types::ChatEventForumTopicToggleIsClosed) -> Self {
        Self::ChatEventForumTopicToggleIsClosed(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventForumTopicToggleIsHidden`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_forum_topic_toggle_is_hidden(val: crate::types::ChatEventForumTopicToggleIsHidden) -> Self {
        Self::ChatEventForumTopicToggleIsHidden(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventForumTopicDeleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_forum_topic_deleted(val: crate::types::ChatEventForumTopicDeleted) -> Self {
        Self::ChatEventForumTopicDeleted(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatEventAction::ChatEventForumTopicPinned`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_forum_topic_pinned(val: crate::types::ChatEventForumTopicPinned) -> Self {
        Self::ChatEventForumTopicPinned(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatEventMessageEdited`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMessageEdited> for ChatEventAction {
    fn from(val: crate::types::ChatEventMessageEdited) -> Self {
        Self::ChatEventMessageEdited(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMessageDeleted`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMessageDeleted> for ChatEventAction {
    fn from(val: crate::types::ChatEventMessageDeleted) -> Self {
        Self::ChatEventMessageDeleted(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMessagePinned`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMessagePinned> for ChatEventAction {
    fn from(val: crate::types::ChatEventMessagePinned) -> Self {
        Self::ChatEventMessagePinned(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMessageUnpinned`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMessageUnpinned> for ChatEventAction {
    fn from(val: crate::types::ChatEventMessageUnpinned) -> Self {
        Self::ChatEventMessageUnpinned(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventPollStopped`] into [`ChatEventAction`].
impl From<crate::types::ChatEventPollStopped> for ChatEventAction {
    fn from(val: crate::types::ChatEventPollStopped) -> Self {
        Self::ChatEventPollStopped(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMemberJoinedByInviteLink`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMemberJoinedByInviteLink> for ChatEventAction {
    fn from(val: crate::types::ChatEventMemberJoinedByInviteLink) -> Self {
        Self::ChatEventMemberJoinedByInviteLink(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMemberJoinedByRequest`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMemberJoinedByRequest> for ChatEventAction {
    fn from(val: crate::types::ChatEventMemberJoinedByRequest) -> Self {
        Self::ChatEventMemberJoinedByRequest(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMemberInvited`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMemberInvited> for ChatEventAction {
    fn from(val: crate::types::ChatEventMemberInvited) -> Self {
        Self::ChatEventMemberInvited(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMemberPromoted`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMemberPromoted> for ChatEventAction {
    fn from(val: crate::types::ChatEventMemberPromoted) -> Self {
        Self::ChatEventMemberPromoted(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMemberRestricted`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMemberRestricted> for ChatEventAction {
    fn from(val: crate::types::ChatEventMemberRestricted) -> Self {
        Self::ChatEventMemberRestricted(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMemberTagChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMemberTagChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventMemberTagChanged) -> Self {
        Self::ChatEventMemberTagChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMemberSubscriptionExtended`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMemberSubscriptionExtended> for ChatEventAction {
    fn from(val: crate::types::ChatEventMemberSubscriptionExtended) -> Self {
        Self::ChatEventMemberSubscriptionExtended(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventAvailableReactionsChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventAvailableReactionsChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventAvailableReactionsChanged) -> Self {
        Self::ChatEventAvailableReactionsChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventBackgroundChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventBackgroundChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventBackgroundChanged) -> Self {
        Self::ChatEventBackgroundChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventDescriptionChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventDescriptionChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventDescriptionChanged) -> Self {
        Self::ChatEventDescriptionChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventEmojiStatusChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventEmojiStatusChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventEmojiStatusChanged) -> Self {
        Self::ChatEventEmojiStatusChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventLinkedChatChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventLinkedChatChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventLinkedChatChanged) -> Self {
        Self::ChatEventLinkedChatChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventLocationChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventLocationChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventLocationChanged) -> Self {
        Self::ChatEventLocationChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventMessageAutoDeleteTimeChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventMessageAutoDeleteTimeChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventMessageAutoDeleteTimeChanged) -> Self {
        Self::ChatEventMessageAutoDeleteTimeChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventPermissionsChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventPermissionsChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventPermissionsChanged) -> Self {
        Self::ChatEventPermissionsChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventPhotoChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventPhotoChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventPhotoChanged) -> Self {
        Self::ChatEventPhotoChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventSlowModeDelayChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventSlowModeDelayChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventSlowModeDelayChanged) -> Self {
        Self::ChatEventSlowModeDelayChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventStickerSetChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventStickerSetChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventStickerSetChanged) -> Self {
        Self::ChatEventStickerSetChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventCustomEmojiStickerSetChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventCustomEmojiStickerSetChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventCustomEmojiStickerSetChanged) -> Self {
        Self::ChatEventCustomEmojiStickerSetChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventTitleChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventTitleChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventTitleChanged) -> Self {
        Self::ChatEventTitleChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventUsernameChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventUsernameChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventUsernameChanged) -> Self {
        Self::ChatEventUsernameChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventActiveUsernamesChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventActiveUsernamesChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventActiveUsernamesChanged) -> Self {
        Self::ChatEventActiveUsernamesChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventAccentColorChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventAccentColorChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventAccentColorChanged) -> Self {
        Self::ChatEventAccentColorChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventProfileAccentColorChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventProfileAccentColorChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventProfileAccentColorChanged) -> Self {
        Self::ChatEventProfileAccentColorChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventHasProtectedContentToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventHasProtectedContentToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventHasProtectedContentToggled) -> Self {
        Self::ChatEventHasProtectedContentToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventInvitesToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventInvitesToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventInvitesToggled) -> Self {
        Self::ChatEventInvitesToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventIsAllHistoryAvailableToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventIsAllHistoryAvailableToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventIsAllHistoryAvailableToggled) -> Self {
        Self::ChatEventIsAllHistoryAvailableToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventHasAggressiveAntiSpamEnabledToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventHasAggressiveAntiSpamEnabledToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventHasAggressiveAntiSpamEnabledToggled) -> Self {
        Self::ChatEventHasAggressiveAntiSpamEnabledToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventSignMessagesToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventSignMessagesToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventSignMessagesToggled) -> Self {
        Self::ChatEventSignMessagesToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventShowMessageSenderToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventShowMessageSenderToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventShowMessageSenderToggled) -> Self {
        Self::ChatEventShowMessageSenderToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventAutomaticTranslationToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventAutomaticTranslationToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventAutomaticTranslationToggled) -> Self {
        Self::ChatEventAutomaticTranslationToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventInviteLinkEdited`] into [`ChatEventAction`].
impl From<crate::types::ChatEventInviteLinkEdited> for ChatEventAction {
    fn from(val: crate::types::ChatEventInviteLinkEdited) -> Self {
        Self::ChatEventInviteLinkEdited(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventInviteLinkRevoked`] into [`ChatEventAction`].
impl From<crate::types::ChatEventInviteLinkRevoked> for ChatEventAction {
    fn from(val: crate::types::ChatEventInviteLinkRevoked) -> Self {
        Self::ChatEventInviteLinkRevoked(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventInviteLinkDeleted`] into [`ChatEventAction`].
impl From<crate::types::ChatEventInviteLinkDeleted> for ChatEventAction {
    fn from(val: crate::types::ChatEventInviteLinkDeleted) -> Self {
        Self::ChatEventInviteLinkDeleted(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventVideoChatCreated`] into [`ChatEventAction`].
impl From<crate::types::ChatEventVideoChatCreated> for ChatEventAction {
    fn from(val: crate::types::ChatEventVideoChatCreated) -> Self {
        Self::ChatEventVideoChatCreated(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventVideoChatEnded`] into [`ChatEventAction`].
impl From<crate::types::ChatEventVideoChatEnded> for ChatEventAction {
    fn from(val: crate::types::ChatEventVideoChatEnded) -> Self {
        Self::ChatEventVideoChatEnded(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventVideoChatMuteNewParticipantsToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventVideoChatMuteNewParticipantsToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventVideoChatMuteNewParticipantsToggled) -> Self {
        Self::ChatEventVideoChatMuteNewParticipantsToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventVideoChatParticipantIsMutedToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventVideoChatParticipantIsMutedToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventVideoChatParticipantIsMutedToggled) -> Self {
        Self::ChatEventVideoChatParticipantIsMutedToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventVideoChatParticipantVolumeLevelChanged`] into [`ChatEventAction`].
impl From<crate::types::ChatEventVideoChatParticipantVolumeLevelChanged> for ChatEventAction {
    fn from(val: crate::types::ChatEventVideoChatParticipantVolumeLevelChanged) -> Self {
        Self::ChatEventVideoChatParticipantVolumeLevelChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventIsForumToggled`] into [`ChatEventAction`].
impl From<crate::types::ChatEventIsForumToggled> for ChatEventAction {
    fn from(val: crate::types::ChatEventIsForumToggled) -> Self {
        Self::ChatEventIsForumToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventForumTopicCreated`] into [`ChatEventAction`].
impl From<crate::types::ChatEventForumTopicCreated> for ChatEventAction {
    fn from(val: crate::types::ChatEventForumTopicCreated) -> Self {
        Self::ChatEventForumTopicCreated(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventForumTopicEdited`] into [`ChatEventAction`].
impl From<crate::types::ChatEventForumTopicEdited> for ChatEventAction {
    fn from(val: crate::types::ChatEventForumTopicEdited) -> Self {
        Self::ChatEventForumTopicEdited(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventForumTopicToggleIsClosed`] into [`ChatEventAction`].
impl From<crate::types::ChatEventForumTopicToggleIsClosed> for ChatEventAction {
    fn from(val: crate::types::ChatEventForumTopicToggleIsClosed) -> Self {
        Self::ChatEventForumTopicToggleIsClosed(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventForumTopicToggleIsHidden`] into [`ChatEventAction`].
impl From<crate::types::ChatEventForumTopicToggleIsHidden> for ChatEventAction {
    fn from(val: crate::types::ChatEventForumTopicToggleIsHidden) -> Self {
        Self::ChatEventForumTopicToggleIsHidden(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventForumTopicDeleted`] into [`ChatEventAction`].
impl From<crate::types::ChatEventForumTopicDeleted> for ChatEventAction {
    fn from(val: crate::types::ChatEventForumTopicDeleted) -> Self {
        Self::ChatEventForumTopicDeleted(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatEventForumTopicPinned`] into [`ChatEventAction`].
impl From<crate::types::ChatEventForumTopicPinned> for ChatEventAction {
    fn from(val: crate::types::ChatEventForumTopicPinned) -> Self {
        Self::ChatEventForumTopicPinned(Box::new(val))
    }
}

/// TDLib `ChatEvent` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEvent {
    /// Represents a chat event
    #[serde(rename(serialize = "chatEvent", deserialize = "chatEvent"))]
    ChatEvent(Box<crate::types::ChatEvent>),
}

impl ChatEvent {
    /// Convenience constructor to create a [`ChatEvent::ChatEvent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event(val: crate::types::ChatEvent) -> Self {
        Self::ChatEvent(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatEvent`] into [`ChatEvent`].
impl From<crate::types::ChatEvent> for ChatEvent {
    fn from(val: crate::types::ChatEvent) -> Self {
        Self::ChatEvent(Box::new(val))
    }
}

/// TDLib `ChatEvents` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEvents {
    /// Contains a list of chat events
    #[serde(rename(serialize = "chatEvents", deserialize = "chatEvents"))]
    ChatEvents(Box<crate::types::ChatEvents>),
}

impl ChatEvents {
    /// Convenience constructor to create a [`ChatEvents::ChatEvents`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_events(val: crate::types::ChatEvents) -> Self {
        Self::ChatEvents(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatEvents`] into [`ChatEvents`].
impl From<crate::types::ChatEvents> for ChatEvents {
    fn from(val: crate::types::ChatEvents) -> Self {
        Self::ChatEvents(Box::new(val))
    }
}

/// TDLib `ChatEventLogFilters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEventLogFilters {
    /// Represents a set of filters used to obtain a chat event log
    #[serde(rename(serialize = "chatEventLogFilters", deserialize = "chatEventLogFilters"))]
    ChatEventLogFilters(Box<crate::types::ChatEventLogFilters>),
}

impl ChatEventLogFilters {
    /// Convenience constructor to create a [`ChatEventLogFilters::ChatEventLogFilters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_event_log_filters(val: crate::types::ChatEventLogFilters) -> Self {
        Self::ChatEventLogFilters(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatEventLogFilters`] into [`ChatEventLogFilters`].
impl From<crate::types::ChatEventLogFilters> for ChatEventLogFilters {
    fn from(val: crate::types::ChatEventLogFilters) -> Self {
        Self::ChatEventLogFilters(Box::new(val))
    }
}

/// TDLib `GiftChatTheme` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftChatTheme {
    /// Describes a chat theme based on an upgraded gift
    #[serde(rename(serialize = "giftChatTheme", deserialize = "giftChatTheme"))]
    GiftChatTheme(Box<crate::types::GiftChatTheme>),
}

impl GiftChatTheme {
    /// Convenience constructor to create a [`GiftChatTheme::GiftChatTheme`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_chat_theme(val: crate::types::GiftChatTheme) -> Self {
        Self::GiftChatTheme(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftChatTheme`] into [`GiftChatTheme`].
impl From<crate::types::GiftChatTheme> for GiftChatTheme {
    fn from(val: crate::types::GiftChatTheme) -> Self {
        Self::GiftChatTheme(Box::new(val))
    }
}

/// TDLib `GiftChatThemes` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftChatThemes {
    /// Contains a list of chat themes based on upgraded gifts
    #[serde(rename(serialize = "giftChatThemes", deserialize = "giftChatThemes"))]
    GiftChatThemes(Box<crate::types::GiftChatThemes>),
}

impl GiftChatThemes {
    /// Convenience constructor to create a [`GiftChatThemes::GiftChatThemes`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_chat_themes(val: crate::types::GiftChatThemes) -> Self {
        Self::GiftChatThemes(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftChatThemes`] into [`GiftChatThemes`].
impl From<crate::types::GiftChatThemes> for GiftChatThemes {
    fn from(val: crate::types::GiftChatThemes) -> Self {
        Self::GiftChatThemes(Box::new(val))
    }
}

/// Describes a chat theme
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatTheme {
    /// A chat theme based on an emoji
    #[serde(rename(serialize = "chatThemeEmoji", deserialize = "chatThemeEmoji"))]
    Emoji(Box<crate::types::ChatThemeEmoji>),
    /// A chat theme based on an upgraded gift
    #[serde(rename(serialize = "chatThemeGift", deserialize = "chatThemeGift"))]
    Gift(Box<crate::types::ChatThemeGift>),
}

impl ChatTheme {
    /// Convenience constructor to create a [`ChatTheme::Emoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji(val: crate::types::ChatThemeEmoji) -> Self {
        Self::Emoji(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatTheme::Gift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift(val: crate::types::ChatThemeGift) -> Self {
        Self::Gift(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatThemeEmoji`] into [`ChatTheme`].
impl From<crate::types::ChatThemeEmoji> for ChatTheme {
    fn from(val: crate::types::ChatThemeEmoji) -> Self {
        Self::Emoji(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatThemeGift`] into [`ChatTheme`].
impl From<crate::types::ChatThemeGift> for ChatTheme {
    fn from(val: crate::types::ChatThemeGift) -> Self {
        Self::Gift(Box::new(val))
    }
}

/// Describes a chat theme to set
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputChatTheme {
    /// A theme based on an emoji
    #[serde(rename(serialize = "inputChatThemeEmoji", deserialize = "inputChatThemeEmoji"))]
    Emoji(Box<crate::types::InputChatThemeEmoji>),
    /// A theme based on an upgraded gift
    #[serde(rename(serialize = "inputChatThemeGift", deserialize = "inputChatThemeGift"))]
    Gift(Box<crate::types::InputChatThemeGift>),
}

impl InputChatTheme {
    /// Convenience constructor to create a [`InputChatTheme::Emoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji(val: crate::types::InputChatThemeEmoji) -> Self {
        Self::Emoji(Box::new(val))
    }

    /// Convenience constructor to create a [`InputChatTheme::Gift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift(val: crate::types::InputChatThemeGift) -> Self {
        Self::Gift(Box::new(val))
    }

}

/// Converts a [`crate::types::InputChatThemeEmoji`] into [`InputChatTheme`].
impl From<crate::types::InputChatThemeEmoji> for InputChatTheme {
    fn from(val: crate::types::InputChatThemeEmoji) -> Self {
        Self::Emoji(Box::new(val))
    }
}

/// Converts a [`crate::types::InputChatThemeGift`] into [`InputChatTheme`].
impl From<crate::types::InputChatThemeGift> for InputChatTheme {
    fn from(val: crate::types::InputChatThemeGift) -> Self {
        Self::Gift(Box::new(val))
    }
}

/// Represents result of checking whether a username can be set for a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CheckChatUsernameResult {
    /// The username can be set
    #[serde(rename(serialize = "checkChatUsernameResultOk", deserialize = "checkChatUsernameResultOk"))]
    Ok,
    /// The username is invalid
    #[serde(rename(serialize = "checkChatUsernameResultUsernameInvalid", deserialize = "checkChatUsernameResultUsernameInvalid"))]
    UsernameInvalid,
    /// The username is occupied
    #[serde(rename(serialize = "checkChatUsernameResultUsernameOccupied", deserialize = "checkChatUsernameResultUsernameOccupied"))]
    UsernameOccupied,
    /// The username can be purchased at https:fragment.com. Information about the username can be received using getCollectibleItemInfo
    #[serde(rename(serialize = "checkChatUsernameResultUsernamePurchasable", deserialize = "checkChatUsernameResultUsernamePurchasable"))]
    UsernamePurchasable,
    /// The user has too many chats with username, one of them must be made private first
    #[serde(rename(serialize = "checkChatUsernameResultPublicChatsTooMany", deserialize = "checkChatUsernameResultPublicChatsTooMany"))]
    PublicChatsTooMany,
    /// The user can't be a member of a public supergroup
    #[serde(rename(serialize = "checkChatUsernameResultPublicGroupsUnavailable", deserialize = "checkChatUsernameResultPublicGroupsUnavailable"))]
    PublicGroupsUnavailable,
}

/// TDLib `NewChatPrivacySettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NewChatPrivacySettings {
    /// Contains privacy settings for chats with non-contacts
    #[serde(rename(serialize = "newChatPrivacySettings", deserialize = "newChatPrivacySettings"))]
    NewChatPrivacySettings(Box<crate::types::NewChatPrivacySettings>),
}

impl NewChatPrivacySettings {
    /// Convenience constructor to create a [`NewChatPrivacySettings::NewChatPrivacySettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_chat_privacy_settings(val: crate::types::NewChatPrivacySettings) -> Self {
        Self::NewChatPrivacySettings(Box::new(val))
    }

}

/// Converts a [`crate::types::NewChatPrivacySettings`] into [`NewChatPrivacySettings`].
impl From<crate::types::NewChatPrivacySettings> for NewChatPrivacySettings {
    fn from(val: crate::types::NewChatPrivacySettings) -> Self {
        Self::NewChatPrivacySettings(Box::new(val))
    }
}

/// Describes result of chat report
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportChatResult {
    /// The chat was reported successfully
    #[serde(rename(serialize = "reportChatResultOk", deserialize = "reportChatResultOk"))]
    Ok,
    /// The user must choose an option to report the chat and repeat request with the chosen option
    #[serde(rename(serialize = "reportChatResultOptionRequired", deserialize = "reportChatResultOptionRequired"))]
    OptionRequired(Box<crate::types::ReportChatResultOptionRequired>),
    /// The user must add additional text details to the report
    #[serde(rename(serialize = "reportChatResultTextRequired", deserialize = "reportChatResultTextRequired"))]
    TextRequired(Box<crate::types::ReportChatResultTextRequired>),
    /// The user must choose messages to report and repeat the reportChat request with the chosen messages
    #[serde(rename(serialize = "reportChatResultMessagesRequired", deserialize = "reportChatResultMessagesRequired"))]
    MessagesRequired,
}

impl ReportChatResult {
    /// Convenience constructor to create a [`ReportChatResult::OptionRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn option_required(val: crate::types::ReportChatResultOptionRequired) -> Self {
        Self::OptionRequired(Box::new(val))
    }

    /// Convenience constructor to create a [`ReportChatResult::TextRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_required(val: crate::types::ReportChatResultTextRequired) -> Self {
        Self::TextRequired(Box::new(val))
    }

}

/// Converts a [`crate::types::ReportChatResultOptionRequired`] into [`ReportChatResult`].
impl From<crate::types::ReportChatResultOptionRequired> for ReportChatResult {
    fn from(val: crate::types::ReportChatResultOptionRequired) -> Self {
        Self::OptionRequired(Box::new(val))
    }
}

/// Converts a [`crate::types::ReportChatResultTextRequired`] into [`ReportChatResult`].
impl From<crate::types::ReportChatResultTextRequired> for ReportChatResult {
    fn from(val: crate::types::ReportChatResultTextRequired) -> Self {
        Self::TextRequired(Box::new(val))
    }
}

/// TDLib `StorageStatisticsByChat` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorageStatisticsByChat {
    /// Contains the storage usage statistics for a specific chat
    #[serde(rename(serialize = "storageStatisticsByChat", deserialize = "storageStatisticsByChat"))]
    StorageStatisticsByChat(Box<crate::types::StorageStatisticsByChat>),
}

impl StorageStatisticsByChat {
    /// Convenience constructor to create a [`StorageStatisticsByChat::StorageStatisticsByChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn storage_statistics_by_chat(val: crate::types::StorageStatisticsByChat) -> Self {
        Self::StorageStatisticsByChat(Box::new(val))
    }

}

/// Converts a [`crate::types::StorageStatisticsByChat`] into [`StorageStatisticsByChat`].
impl From<crate::types::StorageStatisticsByChat> for StorageStatisticsByChat {
    fn from(val: crate::types::StorageStatisticsByChat) -> Self {
        Self::StorageStatisticsByChat(Box::new(val))
    }
}

/// Represents the categories of chats for which a list of frequently used chats can be retrieved
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TopChatCategory {
    /// A category containing frequently used private chats with non-bot users
    #[serde(rename(serialize = "topChatCategoryUsers", deserialize = "topChatCategoryUsers"))]
    Users,
    /// A category containing frequently used private chats with bot users
    #[serde(rename(serialize = "topChatCategoryBots", deserialize = "topChatCategoryBots"))]
    Bots,
    /// A category containing frequently used basic groups and supergroups
    #[serde(rename(serialize = "topChatCategoryGroups", deserialize = "topChatCategoryGroups"))]
    Groups,
    /// A category containing frequently used channels
    #[serde(rename(serialize = "topChatCategoryChannels", deserialize = "topChatCategoryChannels"))]
    Channels,
    /// A category containing frequently used chats with inline bots sorted by their usage in inline mode
    #[serde(rename(serialize = "topChatCategoryInlineBots", deserialize = "topChatCategoryInlineBots"))]
    InlineBots,
    /// A category containing frequently used chats with bots, which were used as guest bots
    #[serde(rename(serialize = "topChatCategoryGuestBots", deserialize = "topChatCategoryGuestBots"))]
    GuestBots,
    /// A category containing frequently used chats with bots, which Web Apps were opened
    #[serde(rename(serialize = "topChatCategoryWebAppBots", deserialize = "topChatCategoryWebAppBots"))]
    WebAppBots,
    /// A category containing frequently used chats used for calls
    #[serde(rename(serialize = "topChatCategoryCalls", deserialize = "topChatCategoryCalls"))]
    Calls,
    /// A category containing frequently used chats used to forward messages
    #[serde(rename(serialize = "topChatCategoryForwardChats", deserialize = "topChatCategoryForwardChats"))]
    ForwardChats,
}

/// Describes type of object, for which statistics are provided
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsObjectType {
    /// Describes a message sent in the chat
    #[serde(rename(serialize = "chatStatisticsObjectTypeMessage", deserialize = "chatStatisticsObjectTypeMessage"))]
    Message(Box<crate::types::ChatStatisticsObjectTypeMessage>),
    /// Describes a story posted on behalf of the chat
    #[serde(rename(serialize = "chatStatisticsObjectTypeStory", deserialize = "chatStatisticsObjectTypeStory"))]
    Story(Box<crate::types::ChatStatisticsObjectTypeStory>),
}

impl ChatStatisticsObjectType {
    /// Convenience constructor to create a [`ChatStatisticsObjectType::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::ChatStatisticsObjectTypeMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatStatisticsObjectType::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::ChatStatisticsObjectTypeStory) -> Self {
        Self::Story(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatStatisticsObjectTypeMessage`] into [`ChatStatisticsObjectType`].
impl From<crate::types::ChatStatisticsObjectTypeMessage> for ChatStatisticsObjectType {
    fn from(val: crate::types::ChatStatisticsObjectTypeMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatStatisticsObjectTypeStory`] into [`ChatStatisticsObjectType`].
impl From<crate::types::ChatStatisticsObjectTypeStory> for ChatStatisticsObjectType {
    fn from(val: crate::types::ChatStatisticsObjectTypeStory) -> Self {
        Self::Story(Box::new(val))
    }
}

/// TDLib `ChatStatisticsInteractionInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsInteractionInfo {
    /// Contains statistics about interactions with a message sent in the chat or a story posted on behalf of the chat
    #[serde(rename(serialize = "chatStatisticsInteractionInfo", deserialize = "chatStatisticsInteractionInfo"))]
    ChatStatisticsInteractionInfo(Box<crate::types::ChatStatisticsInteractionInfo>),
}

impl ChatStatisticsInteractionInfo {
    /// Convenience constructor to create a [`ChatStatisticsInteractionInfo::ChatStatisticsInteractionInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_statistics_interaction_info(val: crate::types::ChatStatisticsInteractionInfo) -> Self {
        Self::ChatStatisticsInteractionInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatStatisticsInteractionInfo`] into [`ChatStatisticsInteractionInfo`].
impl From<crate::types::ChatStatisticsInteractionInfo> for ChatStatisticsInteractionInfo {
    fn from(val: crate::types::ChatStatisticsInteractionInfo) -> Self {
        Self::ChatStatisticsInteractionInfo(Box::new(val))
    }
}

/// TDLib `ChatStatisticsAdministratorActionsInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsAdministratorActionsInfo {
    /// Contains statistics about administrator actions done by a user
    #[serde(rename(serialize = "chatStatisticsAdministratorActionsInfo", deserialize = "chatStatisticsAdministratorActionsInfo"))]
    ChatStatisticsAdministratorActionsInfo(Box<crate::types::ChatStatisticsAdministratorActionsInfo>),
}

impl ChatStatisticsAdministratorActionsInfo {
    /// Convenience constructor to create a [`ChatStatisticsAdministratorActionsInfo::ChatStatisticsAdministratorActionsInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_statistics_administrator_actions_info(val: crate::types::ChatStatisticsAdministratorActionsInfo) -> Self {
        Self::ChatStatisticsAdministratorActionsInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatStatisticsAdministratorActionsInfo`] into [`ChatStatisticsAdministratorActionsInfo`].
impl From<crate::types::ChatStatisticsAdministratorActionsInfo> for ChatStatisticsAdministratorActionsInfo {
    fn from(val: crate::types::ChatStatisticsAdministratorActionsInfo) -> Self {
        Self::ChatStatisticsAdministratorActionsInfo(Box::new(val))
    }
}

/// TDLib `ChatStatisticsInviterInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsInviterInfo {
    /// Contains statistics about number of new members invited by a user
    #[serde(rename(serialize = "chatStatisticsInviterInfo", deserialize = "chatStatisticsInviterInfo"))]
    ChatStatisticsInviterInfo(Box<crate::types::ChatStatisticsInviterInfo>),
}

impl ChatStatisticsInviterInfo {
    /// Convenience constructor to create a [`ChatStatisticsInviterInfo::ChatStatisticsInviterInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_statistics_inviter_info(val: crate::types::ChatStatisticsInviterInfo) -> Self {
        Self::ChatStatisticsInviterInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatStatisticsInviterInfo`] into [`ChatStatisticsInviterInfo`].
impl From<crate::types::ChatStatisticsInviterInfo> for ChatStatisticsInviterInfo {
    fn from(val: crate::types::ChatStatisticsInviterInfo) -> Self {
        Self::ChatStatisticsInviterInfo(Box::new(val))
    }
}

/// Contains a detailed statistics about a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatistics {
    /// A detailed statistics about a supergroup chat
    #[serde(rename(serialize = "chatStatisticsSupergroup", deserialize = "chatStatisticsSupergroup"))]
    Supergroup(Box<crate::types::ChatStatisticsSupergroup>),
    /// A detailed statistics about a channel chat
    #[serde(rename(serialize = "chatStatisticsChannel", deserialize = "chatStatisticsChannel"))]
    Channel(Box<crate::types::ChatStatisticsChannel>),
}

impl ChatStatistics {
    /// Convenience constructor to create a [`ChatStatistics::Supergroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn supergroup(val: crate::types::ChatStatisticsSupergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatStatistics::Channel`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel(val: crate::types::ChatStatisticsChannel) -> Self {
        Self::Channel(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatStatisticsSupergroup`] into [`ChatStatistics`].
impl From<crate::types::ChatStatisticsSupergroup> for ChatStatistics {
    fn from(val: crate::types::ChatStatisticsSupergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatStatisticsChannel`] into [`ChatStatistics`].
impl From<crate::types::ChatStatisticsChannel> for ChatStatistics {
    fn from(val: crate::types::ChatStatisticsChannel) -> Self {
        Self::Channel(Box::new(val))
    }
}


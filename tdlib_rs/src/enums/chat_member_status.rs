#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMemberStatus {
    /// The user is the owner of the chat and has all the administrator privileges
    #[serde(rename(
        serialize = "chatMemberStatusCreator",
        deserialize = "chatMemberStatusCreator"
    ))]
    Creator(crate::types::ChatMemberStatusCreator),
    /// The user is a member of the chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, ban unprivileged members, and manage video chats.
    /// In supergroups and channels, there are more detailed options for administrator privileges
    #[serde(rename(
        serialize = "chatMemberStatusAdministrator",
        deserialize = "chatMemberStatusAdministrator"
    ))]
    Administrator(crate::types::ChatMemberStatusAdministrator),
    /// The user is a member of the chat, without any additional privileges or restrictions
    #[serde(rename(
        serialize = "chatMemberStatusMember",
        deserialize = "chatMemberStatusMember"
    ))]
    Member(crate::types::ChatMemberStatusMember),
    /// The user is under certain restrictions in the chat. Not supported in basic groups and channels
    #[serde(rename(
        serialize = "chatMemberStatusRestricted",
        deserialize = "chatMemberStatusRestricted"
    ))]
    Restricted(crate::types::ChatMemberStatusRestricted),
    /// The user or the chat is not a chat member
    #[serde(rename(
        serialize = "chatMemberStatusLeft",
        deserialize = "chatMemberStatusLeft"
    ))]
    Left,
    /// The user or the chat was banned (and hence is not a member of the chat). Implies the user can't return to the chat, view messages, or be used as a participant identifier to join a video chat of the chat
    #[serde(rename(
        serialize = "chatMemberStatusBanned",
        deserialize = "chatMemberStatusBanned"
    ))]
    Banned(crate::types::ChatMemberStatusBanned),
}

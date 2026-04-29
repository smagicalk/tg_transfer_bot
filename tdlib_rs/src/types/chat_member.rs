#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


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

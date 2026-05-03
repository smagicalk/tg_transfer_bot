#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotCommandScope {
    /// A scope covering all users
    #[serde(rename(
        serialize = "botCommandScopeDefault",
        deserialize = "botCommandScopeDefault"
    ))]
    Default,
    /// A scope covering all private chats
    #[serde(rename(
        serialize = "botCommandScopeAllPrivateChats",
        deserialize = "botCommandScopeAllPrivateChats"
    ))]
    AllPrivateChats,
    /// A scope covering all group and supergroup chats
    #[serde(rename(
        serialize = "botCommandScopeAllGroupChats",
        deserialize = "botCommandScopeAllGroupChats"
    ))]
    AllGroupChats,
    /// A scope covering all group and supergroup chat administrators
    #[serde(rename(
        serialize = "botCommandScopeAllChatAdministrators",
        deserialize = "botCommandScopeAllChatAdministrators"
    ))]
    AllChatAdministrators,
    /// A scope covering all members of a chat
    #[serde(rename(serialize = "botCommandScopeChat", deserialize = "botCommandScopeChat"))]
    Chat(crate::types::BotCommandScopeChat),
    /// A scope covering all administrators of a chat
    #[serde(rename(
        serialize = "botCommandScopeChatAdministrators",
        deserialize = "botCommandScopeChatAdministrators"
    ))]
    ChatAdministrators(crate::types::BotCommandScopeChatAdministrators),
    /// A scope covering a member of a chat
    #[serde(rename(
        serialize = "botCommandScopeChatMember",
        deserialize = "botCommandScopeChatMember"
    ))]
    ChatMember(crate::types::BotCommandScopeChatMember),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TopChatCategory {
    /// A category containing frequently used private chats with non-bot users
    #[serde(rename(
        serialize = "topChatCategoryUsers",
        deserialize = "topChatCategoryUsers"
    ))]
    Users,
    /// A category containing frequently used private chats with bot users
    #[serde(rename(serialize = "topChatCategoryBots", deserialize = "topChatCategoryBots"))]
    Bots,
    /// A category containing frequently used basic groups and supergroups
    #[serde(rename(
        serialize = "topChatCategoryGroups",
        deserialize = "topChatCategoryGroups"
    ))]
    Groups,
    /// A category containing frequently used channels
    #[serde(rename(
        serialize = "topChatCategoryChannels",
        deserialize = "topChatCategoryChannels"
    ))]
    Channels,
    /// A category containing frequently used chats with inline bots sorted by their usage in inline mode
    #[serde(rename(
        serialize = "topChatCategoryInlineBots",
        deserialize = "topChatCategoryInlineBots"
    ))]
    InlineBots,
    /// A category containing frequently used chats with bots, which Web Apps were opened
    #[serde(rename(
        serialize = "topChatCategoryWebAppBots",
        deserialize = "topChatCategoryWebAppBots"
    ))]
    WebAppBots,
    /// A category containing frequently used chats used for calls
    #[serde(rename(
        serialize = "topChatCategoryCalls",
        deserialize = "topChatCategoryCalls"
    ))]
    Calls,
    /// A category containing frequently used chats used to forward messages
    #[serde(rename(
        serialize = "topChatCategoryForwardChats",
        deserialize = "topChatCategoryForwardChats"
    ))]
    ForwardChats,
}

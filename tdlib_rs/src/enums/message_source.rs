#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSource {
    /// The message is from a chat history
    #[serde(rename(
        serialize = "messageSourceChatHistory",
        deserialize = "messageSourceChatHistory"
    ))]
    ChatHistory,
    /// The message is from history of a message thread
    #[serde(rename(
        serialize = "messageSourceMessageThreadHistory",
        deserialize = "messageSourceMessageThreadHistory"
    ))]
    MessageThreadHistory,
    /// The message is from history of a forum topic
    #[serde(rename(
        serialize = "messageSourceForumTopicHistory",
        deserialize = "messageSourceForumTopicHistory"
    ))]
    ForumTopicHistory,
    /// The message is from history of a topic in a channel direct messages chat administered by the current user
    #[serde(rename(
        serialize = "messageSourceDirectMessagesChatTopicHistory",
        deserialize = "messageSourceDirectMessagesChatTopicHistory"
    ))]
    DirectMessagesChatTopicHistory,
    /// The message is from chat, message thread or forum topic history preview
    #[serde(rename(
        serialize = "messageSourceHistoryPreview",
        deserialize = "messageSourceHistoryPreview"
    ))]
    HistoryPreview,
    /// The message is from a chat list or a forum topic list
    #[serde(rename(
        serialize = "messageSourceChatList",
        deserialize = "messageSourceChatList"
    ))]
    ChatList,
    /// The message is from search results, including file downloads, local file list, outgoing document messages, calendar
    #[serde(rename(serialize = "messageSourceSearch", deserialize = "messageSourceSearch"))]
    Search,
    /// The message is from a chat event log
    #[serde(rename(
        serialize = "messageSourceChatEventLog",
        deserialize = "messageSourceChatEventLog"
    ))]
    ChatEventLog,
    /// The message is from a notification
    #[serde(rename(
        serialize = "messageSourceNotification",
        deserialize = "messageSourceNotification"
    ))]
    Notification,
    /// The message was screenshotted; the source must be used only if the message content was visible during the screenshot
    #[serde(rename(
        serialize = "messageSourceScreenshot",
        deserialize = "messageSourceScreenshot"
    ))]
    Screenshot,
    /// The message is from some other source
    #[serde(rename(serialize = "messageSourceOther", deserialize = "messageSourceOther"))]
    Other,
}

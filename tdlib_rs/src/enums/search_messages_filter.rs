#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SearchMessagesFilter {
    /// Returns all found messages, no filter is applied
    #[serde(rename(serialize = "searchMessagesFilterEmpty", deserialize = "searchMessagesFilterEmpty"))]
    Empty,
    /// Returns only animation messages
    #[serde(rename(serialize = "searchMessagesFilterAnimation", deserialize = "searchMessagesFilterAnimation"))]
    Animation,
    /// Returns only audio messages
    #[serde(rename(serialize = "searchMessagesFilterAudio", deserialize = "searchMessagesFilterAudio"))]
    Audio,
    /// Returns only document messages
    #[serde(rename(serialize = "searchMessagesFilterDocument", deserialize = "searchMessagesFilterDocument"))]
    Document,
    /// Returns only photo messages
    #[serde(rename(serialize = "searchMessagesFilterPhoto", deserialize = "searchMessagesFilterPhoto"))]
    Photo,
    /// Returns only video messages
    #[serde(rename(serialize = "searchMessagesFilterVideo", deserialize = "searchMessagesFilterVideo"))]
    Video,
    /// Returns only voice note messages
    #[serde(rename(serialize = "searchMessagesFilterVoiceNote", deserialize = "searchMessagesFilterVoiceNote"))]
    VoiceNote,
    /// Returns only photo and video messages
    #[serde(rename(serialize = "searchMessagesFilterPhotoAndVideo", deserialize = "searchMessagesFilterPhotoAndVideo"))]
    PhotoAndVideo,
    /// Returns only messages containing URLs
    #[serde(rename(serialize = "searchMessagesFilterUrl", deserialize = "searchMessagesFilterUrl"))]
    Url,
    /// Returns only messages containing chat photos
    #[serde(rename(serialize = "searchMessagesFilterChatPhoto", deserialize = "searchMessagesFilterChatPhoto"))]
    ChatPhoto,
    /// Returns only video note messages
    #[serde(rename(serialize = "searchMessagesFilterVideoNote", deserialize = "searchMessagesFilterVideoNote"))]
    VideoNote,
    /// Returns only voice and video note messages
    #[serde(rename(serialize = "searchMessagesFilterVoiceAndVideoNote", deserialize = "searchMessagesFilterVoiceAndVideoNote"))]
    VoiceAndVideoNote,
    /// Returns only messages with mentions of the current user, or messages that are replies to their messages
    #[serde(rename(serialize = "searchMessagesFilterMention", deserialize = "searchMessagesFilterMention"))]
    Mention,
    /// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query, a message thread or by the sending user
    #[serde(rename(serialize = "searchMessagesFilterUnreadMention", deserialize = "searchMessagesFilterUnreadMention"))]
    UnreadMention,
    /// Returns only messages with unread reactions for the current user. When using this filter the results can't be additionally filtered by a query, a message thread or by the sending user
    #[serde(rename(serialize = "searchMessagesFilterUnreadReaction", deserialize = "searchMessagesFilterUnreadReaction"))]
    UnreadReaction,
    /// Returns only failed to send messages. This filter can be used only if the message database is used
    #[serde(rename(serialize = "searchMessagesFilterFailedToSend", deserialize = "searchMessagesFilterFailedToSend"))]
    FailedToSend,
    /// Returns only pinned messages
    #[serde(rename(serialize = "searchMessagesFilterPinned", deserialize = "searchMessagesFilterPinned"))]
    Pinned,
}

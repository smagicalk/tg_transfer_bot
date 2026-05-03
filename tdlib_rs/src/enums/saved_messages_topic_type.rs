#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedMessagesTopicType {
    /// Topic containing messages sent by the current user of forwarded from an unknown chat
    #[serde(rename(
        serialize = "savedMessagesTopicTypeMyNotes",
        deserialize = "savedMessagesTopicTypeMyNotes"
    ))]
    MyNotes,
    /// Topic containing messages forwarded from a user with hidden privacy
    #[serde(rename(
        serialize = "savedMessagesTopicTypeAuthorHidden",
        deserialize = "savedMessagesTopicTypeAuthorHidden"
    ))]
    AuthorHidden,
    /// Topic containing messages forwarded from a specific chat
    #[serde(rename(
        serialize = "savedMessagesTopicTypeSavedFromChat",
        deserialize = "savedMessagesTopicTypeSavedFromChat"
    ))]
    SavedFromChat(crate::types::SavedMessagesTopicTypeSavedFromChat),
}

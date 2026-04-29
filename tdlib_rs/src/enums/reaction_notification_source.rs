#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReactionNotificationSource {
    /// Notifications for reactions are disabled
    #[serde(rename(serialize = "reactionNotificationSourceNone", deserialize = "reactionNotificationSourceNone"))]
    None,
    /// Notifications for reactions are shown only for reactions from contacts
    #[serde(rename(serialize = "reactionNotificationSourceContacts", deserialize = "reactionNotificationSourceContacts"))]
    Contacts,
    /// Notifications for reactions are shown for all reactions
    #[serde(rename(serialize = "reactionNotificationSourceAll", deserialize = "reactionNotificationSourceAll"))]
    All,
}

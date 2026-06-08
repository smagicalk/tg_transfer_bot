#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NewChatPrivacySettings {
    /// Contains privacy settings for chats with non-contacts
    #[serde(rename(
        serialize = "newChatPrivacySettings",
        deserialize = "newChatPrivacySettings"
    ))]
    NewChatPrivacySettings(crate::types::NewChatPrivacySettings),
}

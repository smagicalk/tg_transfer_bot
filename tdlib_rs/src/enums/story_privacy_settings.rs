#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryPrivacySettings {
    /// The story can be viewed by everyone
    #[serde(rename(
        serialize = "storyPrivacySettingsEveryone",
        deserialize = "storyPrivacySettingsEveryone"
    ))]
    Everyone(crate::types::StoryPrivacySettingsEveryone),
    /// The story can be viewed by all contacts except chosen users
    #[serde(rename(
        serialize = "storyPrivacySettingsContacts",
        deserialize = "storyPrivacySettingsContacts"
    ))]
    Contacts(crate::types::StoryPrivacySettingsContacts),
    /// The story can be viewed by all close friends
    #[serde(rename(
        serialize = "storyPrivacySettingsCloseFriends",
        deserialize = "storyPrivacySettingsCloseFriends"
    ))]
    CloseFriends,
    /// The story can be viewed by certain specified users
    #[serde(rename(
        serialize = "storyPrivacySettingsSelectedUsers",
        deserialize = "storyPrivacySettingsSelectedUsers"
    ))]
    SelectedUsers(crate::types::StoryPrivacySettingsSelectedUsers),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The notification settings section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionNotifications {
    /// Subsection of the section; may be one of
    /// "", "accounts", "private-chats", "private-chats/edit", "private-chats/show", "private-chats/preview",
    /// "private-chats/sound", "private-chats/add-exception", "private-chats/delete-exceptions",
    /// "private-chats/light-color", "private-chats/vibrate", "private-chats/priority", "groups", "groups/edit",
    /// "groups/show", "groups/preview", "groups/sound", "groups/add-exception", "groups/delete-exceptions",
    /// "groups/light-color", "groups/vibrate", "groups/priority", "channels", "channels/edit", "channels/show",
    /// "channels/preview", "channels/sound", "channels/add-exception", "channels/delete-exceptions",
    /// "channels/light-color", "channels/vibrate", "channels/priority", "stories", "stories/new", "stories/important",
    /// "stories/show-sender", "stories/sound", "stories/add-exception", "stories/delete-exceptions",
    /// "stories/light-color", "stories/vibrate", "stories/priority", "reactions", "reactions/messages",
    /// "reactions/stories", "reactions/show-sender", "reactions/sound", "reactions/light-color", "reactions/vibrate",
    /// "reactions/priority", "in-app-sounds", "in-app-vibrate", "in-app-preview", "in-chat-sounds", "in-app-popup",
    /// "lock-screen-names", "include-channels", "include-muted-chats", "count-unread-messages", "new-contacts",
    /// "pinned-messages", "reset", "web"
    pub subsection: String,
}

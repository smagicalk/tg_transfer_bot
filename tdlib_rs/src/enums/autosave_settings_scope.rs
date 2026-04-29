#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutosaveSettingsScope {
    /// Autosave settings applied to all private chats without chat-specific settings
    #[serde(rename(serialize = "autosaveSettingsScopePrivateChats", deserialize = "autosaveSettingsScopePrivateChats"))]
    PrivateChats,
    /// Autosave settings applied to all basic group and supergroup chats without chat-specific settings
    #[serde(rename(serialize = "autosaveSettingsScopeGroupChats", deserialize = "autosaveSettingsScopeGroupChats"))]
    GroupChats,
    /// Autosave settings applied to all channel chats without chat-specific settings
    #[serde(rename(serialize = "autosaveSettingsScopeChannelChats", deserialize = "autosaveSettingsScopeChannelChats"))]
    ChannelChats,
    /// Autosave settings applied to a chat
    #[serde(rename(serialize = "autosaveSettingsScopeChat", deserialize = "autosaveSettingsScopeChat"))]
    Chat(crate::types::AutosaveSettingsScopeChat),
}

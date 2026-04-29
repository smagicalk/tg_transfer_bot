#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ScopeNotificationSettings {
    /// Contains information about notification settings for several chats
    #[serde(rename(serialize = "scopeNotificationSettings", deserialize = "scopeNotificationSettings"))]
    ScopeNotificationSettings(crate::types::ScopeNotificationSettings),
}

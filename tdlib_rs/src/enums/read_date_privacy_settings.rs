#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReadDatePrivacySettings {
    /// Contains privacy settings for message read date in private chats. Read dates are always shown to the users that can see online status of the current user regardless of this setting
    #[serde(rename(serialize = "readDatePrivacySettings", deserialize = "readDatePrivacySettings"))]
    ReadDatePrivacySettings(crate::types::ReadDatePrivacySettings),
}

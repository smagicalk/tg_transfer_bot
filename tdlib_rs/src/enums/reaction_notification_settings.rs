#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReactionNotificationSettings {
    /// Contains information about notification settings for reactions
    #[serde(rename(serialize = "reactionNotificationSettings", deserialize = "reactionNotificationSettings"))]
    ReactionNotificationSettings(crate::types::ReactionNotificationSettings),
}

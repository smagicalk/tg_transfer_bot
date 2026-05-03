#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Notification settings for reactions were updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateReactionNotificationSettings {
    /// The new notification settings
    pub notification_settings: crate::types::ReactionNotificationSettings,
}

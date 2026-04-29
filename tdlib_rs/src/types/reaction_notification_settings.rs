#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about notification settings for reactions
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReactionNotificationSettings {
    /// Source of message reactions for which notifications are shown
    pub message_reaction_source: crate::enums::ReactionNotificationSource,
    /// Source of story reactions for which notifications are shown
    pub story_reaction_source: crate::enums::ReactionNotificationSource,
    /// Identifier of the notification sound to be played; 0 if sound is disabled
    #[serde_as(as = "DisplayFromStr")]
    pub sound_id: i64,
    /// True, if reaction sender and emoji must be displayed in notifications
    pub show_preview: bool,
}

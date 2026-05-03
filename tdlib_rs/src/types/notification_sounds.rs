#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of notification sounds
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationSounds {
    /// A list of notification sounds
    pub notification_sounds: Vec<crate::types::NotificationSound>,
}

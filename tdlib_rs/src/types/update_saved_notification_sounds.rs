#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The list of saved notification sounds was updated. This update may not be sent until information about a notification sound was requested for the first time
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedNotificationSounds {
    /// The new list of identifiers of saved notification sounds
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub notification_sound_ids: Vec<i64>,
}

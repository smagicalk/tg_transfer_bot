#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A user in the chat came within proximity alert range
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageProximityAlertTriggered {
    /// The identifier of a user or chat that triggered the proximity alert
    pub traveler_id: crate::enums::MessageSender,
    /// The identifier of a user or chat that subscribed for the proximity alert
    pub watcher_id: crate::enums::MessageSender,
    /// The distance between the users
    pub distance: i32,
}

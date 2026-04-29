#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A user in the chat came within proximity alert range from the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentProximityAlertTriggered {
    /// The distance to the user
    pub distance: i32,
}

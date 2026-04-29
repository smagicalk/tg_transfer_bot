#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageLocation {
    /// The location description
    pub location: crate::types::Location,
    /// Time relative to the message send date, for which the location can be updated, in seconds; if 0x7FFFFFFF, then location can be updated forever
    pub live_period: i32,
    /// Left time for which the location can be updated, in seconds. If 0, then the location can't be updated anymore. The update updateMessageContent is not sent when this field changes
    pub expires_in: i32,
    /// For live locations, a direction in which the location moves, in degrees; 1-360. If 0 the direction is unknown
    pub heading: i32,
    /// For live locations, a maximum distance to another chat member for proximity alerts, in meters (0-100000). 0 if the notification is disabled. Available only to the message sender
    pub proximity_alert_radius: i32,
}

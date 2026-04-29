#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageLocation {
    /// Location to be sent
    pub location: crate::types::Location,
    /// Period for which the location can be updated, in seconds; must be between 60 and 86400 for a temporary live location, 0x7FFFFFFF for permanent live location, and 0 otherwise
    pub live_period: i32,
    /// For live locations, a direction in which the location moves, in degrees; 1-360. Pass 0 if unknown
    pub heading: i32,
    /// For live locations, a maximum distance to another chat member for proximity alerts, in meters (0-100000). Pass 0 if the notification is disabled. Can't be enabled in channels and Saved Messages
    pub proximity_alert_radius: i32,
}

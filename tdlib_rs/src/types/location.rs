#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a location on planet Earth
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Location {
    /// Latitude of the location in degrees; as defined by the sender
    pub latitude: f64,
    /// Longitude of the location, in degrees; as defined by the sender
    pub longitude: f64,
    /// The estimated horizontal accuracy of the location, in meters; as defined by the sender. 0 if unknown
    pub horizontal_accuracy: f64,
}

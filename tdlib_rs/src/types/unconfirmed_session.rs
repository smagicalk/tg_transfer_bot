#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about an unconfirmed session
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UnconfirmedSession {
    /// Session identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Point in time (Unix timestamp) when the user has logged in
    pub log_in_date: i32,
    /// Model of the device that was used for the session creation, as provided by the application
    pub device_model: String,
    /// A human-readable description of the location from which the session was created, based on the IP address
    pub location: String,
}

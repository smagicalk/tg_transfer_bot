#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user is offline
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusOffline {
    /// Point in time (Unix timestamp) when the user was last online
    pub was_online: i32,
}

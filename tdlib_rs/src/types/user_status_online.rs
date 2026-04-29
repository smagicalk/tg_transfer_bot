#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user is online
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusOnline {
    /// Point in time (Unix timestamp) when the user's online status will expire
    pub expires: i32,
}

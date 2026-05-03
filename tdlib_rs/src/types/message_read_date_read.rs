#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains read date of the message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReadDateRead {
    /// Point in time (Unix timestamp) when the message was read by the other user
    pub read_date: i32,
}

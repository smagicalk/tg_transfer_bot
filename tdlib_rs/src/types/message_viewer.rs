#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a viewer of a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageViewer {
    /// User identifier of the viewer
    pub user_id: i64,
    /// Approximate point in time (Unix timestamp) when the message was viewed
    pub view_date: i32,
}

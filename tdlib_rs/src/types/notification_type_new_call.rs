#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// New call was received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationTypeNewCall {
    /// Call identifier
    pub call_id: i32,
}

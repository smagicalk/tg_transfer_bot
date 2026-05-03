#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A notification was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNotification {
    /// Unique notification group identifier
    pub notification_group_id: i32,
    /// Changed notification
    pub notification: crate::types::Notification,
}

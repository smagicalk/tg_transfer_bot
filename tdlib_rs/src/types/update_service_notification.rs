#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateServiceNotification {
    /// Notification type. If type begins with "AUTH_KEY_DROP_", then two buttons "Cancel" and "Log out" must be shown under notification; if user presses the second, all local data must be destroyed using Destroy method
    pub r#type: String,
    /// Notification content
    pub content: crate::enums::MessageContent,
}

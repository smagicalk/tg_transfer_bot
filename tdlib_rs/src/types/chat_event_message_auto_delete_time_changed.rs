#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message auto-delete timer was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessageAutoDeleteTimeChanged {
    /// Previous value of message_auto_delete_time
    pub old_message_auto_delete_time: i32,
    /// New value of message_auto_delete_time
    pub new_message_auto_delete_time: i32,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Some tasks from a checklist were marked as done or not done
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChecklistTasksDone {
    /// Identifier of the message with the checklist; may be 0 or an identifier of a deleted message
    pub checklist_message_id: i64,
    /// Identifiers of tasks that were marked as done
    pub marked_as_done_task_ids: Vec<i32>,
    /// Identifiers of tasks that were marked as not done
    pub marked_as_not_done_task_ids: Vec<i32>,
}

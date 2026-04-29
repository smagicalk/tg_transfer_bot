#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some tasks from a checklist were marked as done or not done
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChecklistTasksDone {
    /// Number of changed tasks
    pub task_count: i32,
}

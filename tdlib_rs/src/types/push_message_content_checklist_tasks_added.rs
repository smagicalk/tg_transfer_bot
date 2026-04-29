#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some tasks were added to a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChecklistTasksAdded {
    /// Number of added tasks
    pub task_count: i32,
}

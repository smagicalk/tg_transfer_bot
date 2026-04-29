#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some tasks were added to a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChecklistTasksAdded {
    /// Identifier of the message with the checklist; may be 0 or an identifier of a deleted message
    pub checklist_message_id: i64,
    /// List of tasks added to the checklist
    pub tasks: Vec<crate::types::ChecklistTask>,
}

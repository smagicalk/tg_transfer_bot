#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChecklistTask {
    /// Describes a task in a checklist
    #[serde(rename(serialize = "checklistTask", deserialize = "checklistTask"))]
    ChecklistTask(crate::types::ChecklistTask),
}

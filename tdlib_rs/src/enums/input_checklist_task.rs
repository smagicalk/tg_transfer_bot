#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputChecklistTask {
    /// Describes a task in a checklist to be sent
    #[serde(rename(serialize = "inputChecklistTask", deserialize = "inputChecklistTask"))]
    InputChecklistTask(crate::types::InputChecklistTask),
}

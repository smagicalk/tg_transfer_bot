#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a task in a checklist to be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive
    pub id: i32,
    /// Text of the task; 1-getOption("checklist_task_text_length_max") characters without line feeds. May contain only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities
    pub text: crate::types::FormattedText,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a checklist to be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChecklist {
    /// Title of the checklist; 1-getOption("checklist_title_length_max") characters. May contain only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities
    pub title: crate::types::FormattedText,
    /// List of tasks in the checklist; 1-getOption("checklist_task_count_max") tasks
    pub tasks: Vec<crate::types::InputChecklistTask>,
    /// True, if other users can add tasks to the list
    pub others_can_add_tasks: bool,
    /// True, if other users can mark tasks as done or not done
    pub others_can_mark_tasks_as_done: bool,
}

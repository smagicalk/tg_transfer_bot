#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Checklist {
    /// Title of the checklist; may contain only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities
    pub title: crate::types::FormattedText,
    /// List of tasks in the checklist
    pub tasks: Vec<crate::types::ChecklistTask>,
    /// True, if users other than creator of the list can add tasks to the list
    pub others_can_add_tasks: bool,
    /// True, if the current user can add tasks to the list if they have Telegram Premium subscription
    pub can_add_tasks: bool,
    /// True, if users other than creator of the list can mark tasks as done or not done. If true, then the checklist is called "group checklist"
    pub others_can_mark_tasks_as_done: bool,
    /// True, if the current user can mark tasks as done or not done if they have Telegram Premium subscription
    pub can_mark_tasks_as_done: bool,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a task in a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task
    pub id: i32,
    /// Text of the task; may contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, Url, EmailAddress, Mention, Hashtag, Cashtag and PhoneNumber entities
    pub text: crate::types::FormattedText,
    /// Identifier of the user or chat that completed the task; may be null if the task isn't completed yet
    pub completed_by: Option<crate::enums::MessageSender>,
    /// Point in time (Unix timestamp) when the task was completed; 0 if the task isn't completed
    pub completion_date: i32,
}

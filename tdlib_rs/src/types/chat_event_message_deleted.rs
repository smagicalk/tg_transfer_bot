#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message was deleted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMessageDeleted {
    /// Deleted message
    pub message: crate::types::Message,
    /// True, if the message deletion can be reported via reportSupergroupAntiSpamFalsePositive
    pub can_report_anti_spam_false_positive: bool,
}

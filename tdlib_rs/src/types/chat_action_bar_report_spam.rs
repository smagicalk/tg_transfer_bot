#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat can be reported as spam using the method reportChat with an empty option_id and message_ids. If the chat is a private chat with a user with an emoji status, then a notice about emoji status usage must be shown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarReportSpam {
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings
    pub can_unarchive: bool,
}

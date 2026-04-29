#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method setMessageSenderBlockList,
/// or the other user can be added to the contact list using the method addContact. If the chat is a private chat with a user with an emoji status, then a notice about emoji status usage must be shown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActionBarReportAddBlock {
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings
    pub can_unarchive: bool,
    /// Basic information about the other user in the chat; may be null if unknown
    pub account_info: Option<crate::types::AccountInfo>,
}

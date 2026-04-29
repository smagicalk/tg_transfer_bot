#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatActionBar {
    /// The chat can be reported as spam using the method reportChat with an empty option_id and message_ids. If the chat is a private chat with a user with an emoji status, then a notice about emoji status usage must be shown
    #[serde(rename(serialize = "chatActionBarReportSpam", deserialize = "chatActionBarReportSpam"))]
    ReportSpam(crate::types::ChatActionBarReportSpam),
    /// The chat is a recently created group chat to which new members can be invited
    #[serde(rename(serialize = "chatActionBarInviteMembers", deserialize = "chatActionBarInviteMembers"))]
    InviteMembers,
    /// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method setMessageSenderBlockList,
    /// or the other user can be added to the contact list using the method addContact. If the chat is a private chat with a user with an emoji status, then a notice about emoji status usage must be shown
    #[serde(rename(serialize = "chatActionBarReportAddBlock", deserialize = "chatActionBarReportAddBlock"))]
    ReportAddBlock(crate::types::ChatActionBarReportAddBlock),
    /// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
    #[serde(rename(serialize = "chatActionBarAddContact", deserialize = "chatActionBarAddContact"))]
    AddContact,
    /// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
    #[serde(rename(serialize = "chatActionBarSharePhoneNumber", deserialize = "chatActionBarSharePhoneNumber"))]
    SharePhoneNumber,
    /// The chat is a private chat with an administrator of a chat to which the user sent join request
    #[serde(rename(serialize = "chatActionBarJoinRequest", deserialize = "chatActionBarJoinRequest"))]
    JoinRequest(crate::types::ChatActionBarJoinRequest),
}

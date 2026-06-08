#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanSendMessageToUserResult {
    /// The user can be messaged
    #[serde(rename(
        serialize = "canSendMessageToUserResultOk",
        deserialize = "canSendMessageToUserResultOk"
    ))]
    Ok,
    /// The user can be messaged, but the messages are paid
    #[serde(rename(
        serialize = "canSendMessageToUserResultUserHasPaidMessages",
        deserialize = "canSendMessageToUserResultUserHasPaidMessages"
    ))]
    UserHasPaidMessages(crate::types::CanSendMessageToUserResultUserHasPaidMessages),
    /// The user can't be messaged, because they are deleted or unknown
    #[serde(rename(
        serialize = "canSendMessageToUserResultUserIsDeleted",
        deserialize = "canSendMessageToUserResultUserIsDeleted"
    ))]
    UserIsDeleted,
    /// The user can't be messaged, because they restrict new chats with non-contacts
    #[serde(rename(
        serialize = "canSendMessageToUserResultUserRestrictsNewChats",
        deserialize = "canSendMessageToUserResultUserRestrictsNewChats"
    ))]
    UserRestrictsNewChats,
}

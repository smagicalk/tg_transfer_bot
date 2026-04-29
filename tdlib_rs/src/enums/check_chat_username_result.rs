#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CheckChatUsernameResult {
    /// The username can be set
    #[serde(rename(serialize = "checkChatUsernameResultOk", deserialize = "checkChatUsernameResultOk"))]
    Ok,
    /// The username is invalid
    #[serde(rename(serialize = "checkChatUsernameResultUsernameInvalid", deserialize = "checkChatUsernameResultUsernameInvalid"))]
    UsernameInvalid,
    /// The username is occupied
    #[serde(rename(serialize = "checkChatUsernameResultUsernameOccupied", deserialize = "checkChatUsernameResultUsernameOccupied"))]
    UsernameOccupied,
    /// The username can be purchased at https:fragment.com. Information about the username can be received using getCollectibleItemInfo
    #[serde(rename(serialize = "checkChatUsernameResultUsernamePurchasable", deserialize = "checkChatUsernameResultUsernamePurchasable"))]
    UsernamePurchasable,
    /// The user has too many chats with username, one of them must be made private first
    #[serde(rename(serialize = "checkChatUsernameResultPublicChatsTooMany", deserialize = "checkChatUsernameResultPublicChatsTooMany"))]
    PublicChatsTooMany,
    /// The user can't be a member of a public supergroup
    #[serde(rename(serialize = "checkChatUsernameResultPublicGroupsUnavailable", deserialize = "checkChatUsernameResultPublicGroupsUnavailable"))]
    PublicGroupsUnavailable,
}

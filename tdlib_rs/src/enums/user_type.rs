#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserType {
    /// A regular user
    #[serde(rename(serialize = "userTypeRegular", deserialize = "userTypeRegular"))]
    Regular,
    /// A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user
    #[serde(rename(serialize = "userTypeDeleted", deserialize = "userTypeDeleted"))]
    Deleted,
    /// A bot (see https:core.telegram.org/bots)
    #[serde(rename(serialize = "userTypeBot", deserialize = "userTypeBot"))]
    Bot(crate::types::UserTypeBot),
    /// No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type
    #[serde(rename(serialize = "userTypeUnknown", deserialize = "userTypeUnknown"))]
    Unknown,
}

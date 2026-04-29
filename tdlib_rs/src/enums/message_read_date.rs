#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReadDate {
    /// Contains read date of the message
    #[serde(rename(serialize = "messageReadDateRead", deserialize = "messageReadDateRead"))]
    Read(crate::types::MessageReadDateRead),
    /// The message is unread yet
    #[serde(rename(serialize = "messageReadDateUnread", deserialize = "messageReadDateUnread"))]
    Unread,
    /// The message is too old to get read date
    #[serde(rename(serialize = "messageReadDateTooOld", deserialize = "messageReadDateTooOld"))]
    TooOld,
    /// The read date is unknown due to privacy settings of the other user
    #[serde(rename(serialize = "messageReadDateUserPrivacyRestricted", deserialize = "messageReadDateUserPrivacyRestricted"))]
    UserPrivacyRestricted,
    /// The read date is unknown due to privacy settings of the current user, but will be known if the user subscribes to Telegram Premium
    #[serde(rename(serialize = "messageReadDateMyPrivacyRestricted", deserialize = "messageReadDateMyPrivacyRestricted"))]
    MyPrivacyRestricted,
}

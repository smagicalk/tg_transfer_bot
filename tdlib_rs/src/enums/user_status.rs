#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserStatus {
    /// The user's status has never been changed
    #[serde(rename(serialize = "userStatusEmpty", deserialize = "userStatusEmpty"))]
    Empty,
    /// The user is online
    #[serde(rename(serialize = "userStatusOnline", deserialize = "userStatusOnline"))]
    Online(crate::types::UserStatusOnline),
    /// The user is offline
    #[serde(rename(serialize = "userStatusOffline", deserialize = "userStatusOffline"))]
    Offline(crate::types::UserStatusOffline),
    /// The user was online recently
    #[serde(rename(serialize = "userStatusRecently", deserialize = "userStatusRecently"))]
    Recently(crate::types::UserStatusRecently),
    /// The user is offline, but was online last week
    #[serde(rename(serialize = "userStatusLastWeek", deserialize = "userStatusLastWeek"))]
    LastWeek(crate::types::UserStatusLastWeek),
    /// The user is offline, but was online last month
    #[serde(rename(serialize = "userStatusLastMonth", deserialize = "userStatusLastMonth"))]
    LastMonth(crate::types::UserStatusLastMonth),
}

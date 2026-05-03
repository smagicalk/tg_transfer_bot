#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Notification {
    /// Contains information about a notification
    #[serde(rename(serialize = "notification", deserialize = "notification"))]
    Notification(crate::types::Notification),
}

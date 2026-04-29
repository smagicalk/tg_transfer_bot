#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PushReceiverId {
    /// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
    #[serde(rename(serialize = "pushReceiverId", deserialize = "pushReceiverId"))]
    PushReceiverId(crate::types::PushReceiverId),
}

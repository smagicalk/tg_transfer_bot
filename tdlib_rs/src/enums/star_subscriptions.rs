#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarSubscriptions {
    /// Represents a list of Telegram Star subscriptions
    #[serde(rename(serialize = "starSubscriptions", deserialize = "starSubscriptions"))]
    StarSubscriptions(crate::types::StarSubscriptions),
}

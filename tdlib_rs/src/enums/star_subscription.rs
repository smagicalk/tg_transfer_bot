#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarSubscription {
    /// Contains information about subscription to a channel chat, a bot, or a business account that was paid in Telegram Stars
    #[serde(rename(serialize = "starSubscription", deserialize = "starSubscription"))]
    StarSubscription(crate::types::StarSubscription),
}

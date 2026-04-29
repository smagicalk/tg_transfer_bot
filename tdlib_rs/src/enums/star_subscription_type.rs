#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarSubscriptionType {
    /// Describes a subscription to a channel chat
    #[serde(rename(serialize = "starSubscriptionTypeChannel", deserialize = "starSubscriptionTypeChannel"))]
    Channel(crate::types::StarSubscriptionTypeChannel),
    /// Describes a subscription in a bot or a business account
    #[serde(rename(serialize = "starSubscriptionTypeBot", deserialize = "starSubscriptionTypeBot"))]
    Bot(crate::types::StarSubscriptionTypeBot),
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessGreetingMessageSettings {
    /// Describes settings for greeting messages that are automatically sent by a Telegram Business account as response to incoming messages in an inactive private chat
    #[serde(rename(
        serialize = "businessGreetingMessageSettings",
        deserialize = "businessGreetingMessageSettings"
    ))]
    BusinessGreetingMessageSettings(crate::types::BusinessGreetingMessageSettings),
}

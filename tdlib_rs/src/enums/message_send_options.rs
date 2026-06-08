#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSendOptions {
    /// Options to be used when a message is sent
    #[serde(rename(serialize = "messageSendOptions", deserialize = "messageSendOptions"))]
    MessageSendOptions(crate::types::MessageSendOptions),
}

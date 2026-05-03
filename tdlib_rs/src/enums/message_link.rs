#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageLink {
    /// Contains an HTTPS link to a message in a supergroup or channel, or a forum topic
    #[serde(rename(serialize = "messageLink", deserialize = "messageLink"))]
    MessageLink(crate::types::MessageLink),
}

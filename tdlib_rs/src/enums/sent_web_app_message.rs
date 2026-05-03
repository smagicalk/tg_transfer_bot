#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SentWebAppMessage {
    /// Information about the message sent by answerWebAppQuery
    #[serde(rename(serialize = "sentWebAppMessage", deserialize = "sentWebAppMessage"))]
    SentWebAppMessage(crate::types::SentWebAppMessage),
}

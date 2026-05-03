#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallbackQueryAnswer {
    /// Contains a bot's answer to a callback query
    #[serde(rename(serialize = "callbackQueryAnswer", deserialize = "callbackQueryAnswer"))]
    CallbackQueryAnswer(crate::types::CallbackQueryAnswer),
}

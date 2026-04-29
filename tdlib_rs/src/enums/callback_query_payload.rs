#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallbackQueryPayload {
    /// The payload for a general callback button
    #[serde(rename(serialize = "callbackQueryPayloadData", deserialize = "callbackQueryPayloadData"))]
    Data(crate::types::CallbackQueryPayloadData),
    /// The payload for a callback button requiring password
    #[serde(rename(serialize = "callbackQueryPayloadDataWithPassword", deserialize = "callbackQueryPayloadDataWithPassword"))]
    DataWithPassword(crate::types::CallbackQueryPayloadDataWithPassword),
    /// The payload for a game callback button
    #[serde(rename(serialize = "callbackQueryPayloadGame", deserialize = "callbackQueryPayloadGame"))]
    Game(crate::types::CallbackQueryPayloadGame),
}

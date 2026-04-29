#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageAutoDeleteTime {
    /// Contains default auto-delete timer setting for new chats
    #[serde(rename(serialize = "messageAutoDeleteTime", deserialize = "messageAutoDeleteTime"))]
    MessageAutoDeleteTime(crate::types::MessageAutoDeleteTime),
}

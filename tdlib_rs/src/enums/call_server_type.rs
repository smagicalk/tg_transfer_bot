#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallServerType {
    /// A Telegram call reflector
    #[serde(rename(
        serialize = "callServerTypeTelegramReflector",
        deserialize = "callServerTypeTelegramReflector"
    ))]
    TelegramReflector(crate::types::CallServerTypeTelegramReflector),
    /// A WebRTC server
    #[serde(rename(
        serialize = "callServerTypeWebrtc",
        deserialize = "callServerTypeWebrtc"
    ))]
    Webrtc(crate::types::CallServerTypeWebrtc),
}

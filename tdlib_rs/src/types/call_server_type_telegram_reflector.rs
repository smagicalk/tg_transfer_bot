#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram call reflector
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallServerTypeTelegramReflector {
    /// A peer tag to be used with the reflector
    pub peer_tag: String,
    /// True, if the server uses TCP instead of UDP
    pub is_tcp: bool,
}

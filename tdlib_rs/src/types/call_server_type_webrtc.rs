#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A WebRTC server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallServerTypeWebrtc {
    /// Username to be used for authentication
    pub username: String,
    /// Authentication password
    pub password: String,
    /// True, if the server supports TURN
    pub supports_turn: bool,
    /// True, if the server supports STUN
    pub supports_stun: bool,
}

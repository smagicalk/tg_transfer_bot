#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The call is ready to use
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallStateReady {
    /// Call protocols supported by the other call participant
    pub protocol: crate::types::CallProtocol,
    /// List of available call servers
    pub servers: Vec<crate::types::CallServer>,
    /// A JSON-encoded call config
    pub config: String,
    /// Call encryption key
    pub encryption_key: String,
    /// Encryption key fingerprint represented as 4 emoji
    pub emojis: Vec<String>,
    /// True, if peer-to-peer connection is allowed by users privacy settings
    pub allow_p2p: bool,
    /// True, if the other party supports upgrading of the call to a group call
    pub is_group_call_supported: bool,
    /// Custom JSON-encoded call parameters to be passed to tgcalls
    pub custom_parameters: String,
}

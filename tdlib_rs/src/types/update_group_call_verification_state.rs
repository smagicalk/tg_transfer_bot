#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The verification state of an encrypted group call has changed; for group calls not bound to a chat only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallVerificationState {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// The call state generation to which the emoji corresponds. If generation is different for two users, then their emoji may be also different
    pub generation: i32,
    /// Group call state fingerprint represented as 4 emoji; may be empty if the state isn't verified yet
    pub emojis: Vec<String>,
}

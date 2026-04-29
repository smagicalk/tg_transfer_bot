#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The session was created recently, user needs to wait
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanTransferOwnershipResultSessionTooFresh {
    /// Time left before the session can be used to transfer ownership of a chat, in seconds
    pub retry_after: i32,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The call is pending, waiting to be accepted by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallStatePending {
    /// True, if the call has already been created by the server
    pub is_created: bool,
    /// True, if the call has already been received by the other party
    pub is_received: bool,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The message is being sent now, but has not yet been delivered to the server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSendingStatePending {
    /// Non-persistent message sending identifier, specified by the application
    pub sending_id: i32,
}

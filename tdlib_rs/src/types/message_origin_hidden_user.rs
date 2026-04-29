#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The message was originally sent by a user, which is hidden by their privacy settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginHiddenUser {
    /// Name of the sender
    pub sender_name: String,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a chat shared with a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SharedChat {
    /// Chat identifier
    pub chat_id: i64,
}

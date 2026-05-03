#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message will be self-destructed in the specified time after its content was opened
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSelfDestructTypeTimer {
    /// The message's self-destruct time, in seconds; must be between 0 and 60 in private chats
    pub self_destruct_time: i32,
}

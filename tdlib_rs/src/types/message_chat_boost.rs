#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat was boosted by the sender of the message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatBoost {
    /// Number of times the chat was boosted
    pub boost_count: i32,
}

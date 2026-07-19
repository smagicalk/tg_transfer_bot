#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat boost has changed; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatBoost {
    /// Chat identifier
    pub chat_id: i64,
    /// New information about the boost
    pub boost: crate::types::ChatBoost,
}

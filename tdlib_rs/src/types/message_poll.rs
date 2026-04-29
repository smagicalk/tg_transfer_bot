#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a poll
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessagePoll {
    /// The poll description
    pub poll: crate::types::Poll,
}

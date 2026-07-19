#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A poll was updated; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdatePoll {
    /// New data about the poll
    pub poll: crate::types::Poll,
}

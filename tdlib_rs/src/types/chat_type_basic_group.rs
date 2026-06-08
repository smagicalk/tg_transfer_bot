#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A basic group (a chat with 0-200 other users)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatTypeBasicGroup {
    /// Basic group identifier
    pub basic_group_id: i64,
}

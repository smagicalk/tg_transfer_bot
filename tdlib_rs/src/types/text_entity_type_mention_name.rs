#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A text shows instead of a raw mention of the user (e.g., when the user has no username)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeMentionName {
    /// Identifier of the mentioned user
    pub user_id: i64,
}

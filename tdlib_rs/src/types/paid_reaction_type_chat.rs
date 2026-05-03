#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A paid reaction on behalf of an owned chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidReactionTypeChat {
    /// Identifier of the chat
    pub chat_id: i64,
}

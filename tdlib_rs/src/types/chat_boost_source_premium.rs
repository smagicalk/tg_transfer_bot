#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A user with Telegram Premium subscription or gifted Telegram Premium boosted the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSourcePremium {
    /// Identifier of the user
    pub user_id: i64,
}

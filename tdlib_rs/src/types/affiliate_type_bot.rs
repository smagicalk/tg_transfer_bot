#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The affiliate is a bot owned by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateTypeBot {
    /// User identifier of the bot
    pub user_id: i64,
}

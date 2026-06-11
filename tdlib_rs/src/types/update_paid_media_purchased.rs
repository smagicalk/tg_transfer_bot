#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Paid media were purchased by a user; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdatePaidMediaPurchased {
    /// User identifier
    pub user_id: i64,
    /// Bot-specified payload for the paid media
    pub payload: String,
}

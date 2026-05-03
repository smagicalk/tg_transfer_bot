#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Suggests the user to extend their expiring Telegram Premium subscription
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionExtendPremium {
    /// A URL for managing Telegram Premium subscription
    pub manage_premium_subscription_url: String,
}

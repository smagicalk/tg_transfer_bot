#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user subscribing to Telegram Premium
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorePaymentPurposePremiumSubscription {
    /// Pass true if this is a restore of a Telegram Premium purchase; only for App Store
    pub is_restore: bool,
    /// Pass true if this is an upgrade from a monthly subscription to early subscription; only for App Store
    pub is_upgrade: bool,
}

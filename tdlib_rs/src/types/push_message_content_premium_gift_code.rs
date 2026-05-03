#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with a Telegram Premium gift code created for the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentPremiumGiftCode {
    /// Number of months the Telegram Premium subscription will be active after code activation
    pub month_count: i32,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes subscription plan paid in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarSubscriptionPricing {
    /// The number of seconds between consecutive Telegram Star debiting
    pub period: i32,
    /// The Telegram Star amount that must be paid for each period
    pub star_count: i64,
}

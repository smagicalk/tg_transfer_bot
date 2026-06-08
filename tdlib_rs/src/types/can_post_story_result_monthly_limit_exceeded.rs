#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The monthly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultMonthlyLimitExceeded {
    /// Time left before the user can post the next story, in seconds
    pub retry_after: i32,
}

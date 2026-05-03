#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about an active affiliate program
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AffiliateProgramInfo {
    /// Parameters of the affiliate program
    pub parameters: crate::types::AffiliateProgramParameters,
    /// Point in time (Unix timestamp) when the affiliate program will be closed; 0 if the affiliate program isn't scheduled to be closed.
    /// If positive, then the program can't be connected using connectAffiliateProgram, but active connections will work until the date
    pub end_date: i32,
    /// The amount of daily revenue per user in Telegram Stars of the bot that created the affiliate program
    pub daily_revenue_per_user_amount: crate::types::StarAmount,
}

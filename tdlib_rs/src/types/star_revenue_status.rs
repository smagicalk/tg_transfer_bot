#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about Telegram Stars earned by a user or a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarRevenueStatus {
    /// Total Telegram Star amount earned
    pub total_amount: crate::types::StarAmount,
    /// The Telegram Star amount that isn't withdrawn yet
    pub current_amount: crate::types::StarAmount,
    /// The Telegram Star amount that is available for withdrawal
    pub available_amount: crate::types::StarAmount,
    /// True, if Telegram Stars can be withdrawn now or later
    pub withdrawal_enabled: bool,
    /// Time left before the next withdrawal can be started, in seconds; 0 if withdrawal can be started now
    pub next_withdrawal_in: i32,
}

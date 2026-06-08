#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes state of the stake dice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StakeDiceState {
    /// Hash of the state to use for sending the next dice; may be empty if the stake dice can't be sent by the current user
    pub state_hash: String,
    /// The Toncoin amount that was staked in the previous roll; in the smallest units of the currency
    pub stake_toncoin_amount: i64,
    /// The amounts of Toncoins that are suggested to be staked; in the smallest units of the currency
    pub suggested_stake_toncoin_amounts: Vec<i64>,
    /// The number of rolled sixes towards the streak; 0-2
    pub current_streak: i32,
    /// The number of Toncoins received by the user for each 1000 Toncoins staked if the dice outcome is 1-6 correspondingly; may be empty if the stake dice can't be sent by the current user
    pub prize_per_mille: Vec<i32>,
    /// The number of Toncoins received by the user for each 1000 Toncoins staked if the dice outcome is 6 three times in a row with the same stake
    pub streak_prize_per_mille: i32,
}

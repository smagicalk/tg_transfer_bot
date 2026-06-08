#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A stake dice message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageStakeDice {
    /// Hash of the stake dice state. The state hash can be used only if it was received recently enough. Otherwise, a new state must be requested using getStakeDiceState
    pub state_hash: String,
    /// The Toncoin amount that will be staked; in the smallest units of the currency. Must be in the range
    /// getOption("stake_dice_stake_amount_min")-getOption("stake_dice_stake_amount_max")
    pub stake_toncoin_amount: i64,
    /// True, if the chat message draft must be deleted
    pub clear_draft: bool,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The number of Toncoins owned by the current user has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateOwnedTonCount {
    /// The new amount of owned Toncoins; in the smallest units of the cryptocurrency
    pub ton_amount: i64,
}

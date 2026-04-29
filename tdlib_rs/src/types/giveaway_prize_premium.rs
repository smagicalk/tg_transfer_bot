#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The giveaway sends Telegram Premium subscriptions to the winners
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayPrizePremium {
    /// Number of months the Telegram Premium subscription will be active after code activation
    pub month_count: i32,
}

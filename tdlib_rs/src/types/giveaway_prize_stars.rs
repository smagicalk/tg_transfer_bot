#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The giveaway sends Telegram Stars to the winners
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayPrizeStars {
    /// Number of Telegram Stars that will be shared by all winners
    pub star_count: i64,
}

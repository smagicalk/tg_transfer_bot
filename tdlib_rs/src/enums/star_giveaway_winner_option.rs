#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarGiveawayWinnerOption {
    /// Describes an option for the number of winners of a Telegram Star giveaway
    #[serde(rename(serialize = "starGiveawayWinnerOption", deserialize = "starGiveawayWinnerOption"))]
    StarGiveawayWinnerOption(crate::types::StarGiveawayWinnerOption),
}

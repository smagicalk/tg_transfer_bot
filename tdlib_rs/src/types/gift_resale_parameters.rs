#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes parameters of a unique gift available for resale
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftResaleParameters {
    /// Resale price of the gift in Telegram Stars
    pub star_count: i64,
    /// Resale price of the gift in 1/100 of Toncoin
    pub toncoin_cent_count: i64,
    /// True, if the gift can be bought only using Toncoins
    pub toncoin_only: bool,
}

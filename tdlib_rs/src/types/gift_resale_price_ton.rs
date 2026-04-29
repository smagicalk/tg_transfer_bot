#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes price of a resold gift in Toncoins
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftResalePriceTon {
    /// The amount of 1/100 of Toncoin expected to be paid for the gift. Must be in the range
    /// getOption("gift_resale_toncoin_cent_count_min")-getOption("gift_resale_toncoin_cent_count_max")
    pub toncoin_cent_count: i64,
}

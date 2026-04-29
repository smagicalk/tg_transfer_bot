#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes price of a suggested post in Toncoins
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostPriceTon {
    /// The amount of 1/100 of Toncoin expected to be paid for the post; getOption("suggested_post_toncoin_cent_count_min")-getOption("suggested_post_toncoin_cent_count_max")
    pub toncoin_cent_count: i64,
}

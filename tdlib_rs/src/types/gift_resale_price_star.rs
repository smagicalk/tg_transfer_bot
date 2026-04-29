#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes price of a resold gift in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftResalePriceStar {
    /// The Telegram Star amount expected to be paid for the gift. Must be in the range
    /// getOption("gift_resale_star_count_min")-getOption("gift_resale_star_count_max") for gifts put for resale
    pub star_count: i64,
}

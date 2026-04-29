#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes price of a suggested post in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostPriceStar {
    /// The Telegram Star amount expected to be paid for the post; getOption("suggested_post_star_count_min")-getOption("suggested_post_star_count_max")
    pub star_count: i64,
}

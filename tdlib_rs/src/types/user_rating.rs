#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains description of user rating
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserRating {
    /// The level of the user; may be negative
    pub level: i32,
    /// True, if the maximum level is reached
    pub is_maximum_level_reached: bool,
    /// Numerical value of the rating
    pub rating: i64,
    /// The rating required for the current level
    pub current_level_rating: i64,
    /// The rating required for the next level; 0 if the maximum level is reached
    pub next_level_rating: i64,
}

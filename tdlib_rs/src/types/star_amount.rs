#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a possibly non-integer Telegram Star amount
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarAmount {
    /// The integer Telegram Star amount rounded to 0
    pub star_count: i64,
    /// The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999
    pub nanostar_count: i32,
}

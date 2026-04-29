#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Crafting isn't possible because one of the gifts can't be used for crafting yet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CraftGiftResultTooEarly {
    /// Time left before the gift can be used for crafting
    pub retry_after: i32,
}

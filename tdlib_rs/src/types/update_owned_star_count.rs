#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The number of Telegram Stars owned by the current user has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateOwnedStarCount {
    /// The new amount of owned Telegram Stars
    pub star_amount: crate::types::StarAmount,
}

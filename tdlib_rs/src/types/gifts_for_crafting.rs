#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of gifts received by a user or a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftsForCrafting {
    /// The total number of received gifts
    pub total_count: i32,
    /// The list of gifts
    pub gifts: Vec<crate::types::ReceivedGift>,
    /// The 4 objects that describe probabilities of the crafted gift to have the backdrop or symbol of one of the original gifts
    /// for the cases when 1, 2, 3 or 4 gifts are used in the craft correspondingly
    pub attribute_persistence_probabilities: Vec<crate::types::AttributeCraftPersistenceProbability>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes chance of the crafted gift to have the backdrop or symbol of one of the original gifts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AttributeCraftPersistenceProbability {
    /// The 4 numbers that describe probability of the craft result to have the same attribute as one of the original gifts
    /// if 1, 2, 3, or 4 gifts with the attribute are used in the craft. Each number represents the number of crafted gifts with the original attribute per 1000 successful craftings
    pub persistence_chance_per_mille: Vec<i32>,
}

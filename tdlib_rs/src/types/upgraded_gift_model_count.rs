#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a model of an upgraded gift with the number of gifts found
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftModelCount {
    /// The model
    pub model: crate::types::UpgradedGiftModel,
    /// Total number of gifts with the model
    pub total_count: i32,
}

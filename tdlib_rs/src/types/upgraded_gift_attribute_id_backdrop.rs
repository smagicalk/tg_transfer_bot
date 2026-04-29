#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Identifier of a gift backdrop
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftAttributeIdBackdrop {
    /// Identifier of the backdrop
    pub backdrop_id: i32,
}

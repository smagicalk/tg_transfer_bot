#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An area with an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeUpgradedGift {
    /// Unique name of the upgraded gift
    pub gift_name: String,
}

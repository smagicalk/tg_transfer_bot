#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of chat boost slots
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSlots {
    /// List of boost slots
    pub slots: Vec<crate::types::ChatBoostSlot>,
}

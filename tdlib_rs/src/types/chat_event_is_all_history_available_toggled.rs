#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The is_all_history_available setting of a supergroup was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventIsAllHistoryAvailableToggled {
    /// New value of is_all_history_available
    pub is_all_history_available: bool,
}

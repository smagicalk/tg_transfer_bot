#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of suggested to the user actions has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSuggestedActions {
    /// Added suggested actions
    pub added_actions: Vec<crate::enums::SuggestedAction>,
    /// Removed suggested actions
    pub removed_actions: Vec<crate::enums::SuggestedAction>,
}

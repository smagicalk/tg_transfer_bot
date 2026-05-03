#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The list of saved animations was updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedAnimations {
    /// The new list of file identifiers of saved animations
    pub animation_ids: Vec<i32>,
}

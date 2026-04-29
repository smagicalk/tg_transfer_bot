#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of updates
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Updates {
    /// List of updates
    pub updates: Vec<crate::enums::Update>,
}

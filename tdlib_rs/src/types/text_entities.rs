#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of text entities
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntities {
    /// List of text entities
    pub entities: Vec<crate::types::TextEntity>,
}

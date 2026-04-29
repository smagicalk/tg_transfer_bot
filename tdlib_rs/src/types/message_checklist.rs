#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChecklist {
    /// The checklist description
    pub list: crate::types::Checklist,
}

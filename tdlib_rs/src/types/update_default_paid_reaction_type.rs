#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The type of default paid reaction has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateDefaultPaidReactionType {
    /// The new type of the default paid reaction
    pub r#type: crate::enums::PaidReactionType,
}

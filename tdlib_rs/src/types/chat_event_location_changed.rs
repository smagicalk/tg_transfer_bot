#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The supergroup location was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventLocationChanged {
    /// Previous location; may be null
    pub old_location: Option<crate::types::ChatLocation>,
    /// New location; may be null
    pub new_location: Option<crate::types::ChatLocation>,
}

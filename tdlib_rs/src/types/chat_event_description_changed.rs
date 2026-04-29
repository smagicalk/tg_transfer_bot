#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat description was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventDescriptionChanged {
    /// Previous chat description
    pub old_description: String,
    /// New chat description
    pub new_description: String,
}

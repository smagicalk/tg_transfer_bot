#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The has_protected_content setting of a channel was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventHasProtectedContentToggled {
    /// New value of has_protected_content
    pub has_protected_content: bool,
}

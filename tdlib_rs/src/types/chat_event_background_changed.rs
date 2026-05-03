#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat background was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventBackgroundChanged {
    /// Previous background; may be null if none
    pub old_background: Option<crate::types::ChatBackground>,
    /// New background; may be null if none
    pub new_background: Option<crate::types::ChatBackground>,
}

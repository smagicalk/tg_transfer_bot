#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to boost a supergroup chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeSupergroupBoost {
    /// Photo of the chat; may be null
    pub photo: Option<crate::types::ChatPhoto>,
}

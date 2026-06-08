#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to boost a channel chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeChannelBoost {
    /// Photo of the chat; may be null
    pub photo: Option<crate::types::ChatPhoto>,
}

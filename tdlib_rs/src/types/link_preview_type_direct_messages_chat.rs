#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a direct messages chat of a channel
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeDirectMessagesChat {
    /// Photo of the channel chat; may be null
    pub photo: Option<crate::types::ChatPhoto>,
}

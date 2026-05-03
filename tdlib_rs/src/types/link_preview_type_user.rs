#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeUser {
    /// Photo of the user; may be null if none
    pub photo: Option<crate::types::ChatPhoto>,
    /// True, if the user is a bot
    pub is_bot: bool,
}

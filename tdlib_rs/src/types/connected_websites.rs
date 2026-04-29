#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of websites the current user is logged in with Telegram
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedWebsites {
    /// List of connected websites
    pub websites: Vec<crate::types::ConnectedWebsite>,
}

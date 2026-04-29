#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains an HTTPS link to boost a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostLink {
    /// The link
    pub link: String,
    /// True, if the link will work for non-members of the chat
    pub is_public: bool,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a business chat. Use getBusinessChatLinkInfo with the provided link name to get information about the link,
/// then open received private chat and replace chat draft with the provided text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBusinessChat {
    /// Name of the link
    pub link_name: String,
}

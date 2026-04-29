#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is an affiliate program link. Call searchChatAffiliateProgram with the given username and referrer to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatAffiliateProgram {
    /// Username to be passed to searchChatAffiliateProgram
    pub username: String,
    /// Referrer to be passed to searchChatAffiliateProgram
    pub referrer: String,
}

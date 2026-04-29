#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a sponsored chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SponsoredChat {
    /// Unique identifier of this result
    pub unique_id: i64,
    /// Chat identifier
    pub chat_id: i64,
    /// Additional optional information about the sponsor to be shown along with the chat
    pub sponsor_info: String,
    /// If non-empty, additional information about the sponsored chat to be shown along with the chat
    pub additional_info: String,
}

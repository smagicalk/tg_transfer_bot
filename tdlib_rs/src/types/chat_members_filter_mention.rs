#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Returns users which can be mentioned in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembersFilterMention {
    /// Identifier of the topic in which the users will be mentioned; pass null if none
    pub topic_id: Option<crate::enums::MessageTopic>,
}

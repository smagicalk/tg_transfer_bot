#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Translation of chat messages was enabled or disabled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatIsTranslatable {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of is_translatable
    pub is_translatable: bool,
}

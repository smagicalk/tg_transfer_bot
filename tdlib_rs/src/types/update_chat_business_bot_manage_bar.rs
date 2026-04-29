#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The bar for managing business bot was changed in a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatBusinessBotManageBar {
    /// Chat identifier
    pub chat_id: i64,
    /// The new value of the business bot manage bar; may be null
    pub business_bot_manage_bar: Option<crate::types::BusinessBotManageBar>,
}

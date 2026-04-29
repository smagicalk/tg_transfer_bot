#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A button with a user reference to be handled in the same way as textEntityTypeMentionName entities
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeUser {
    /// User identifier
    pub user_id: i64,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a game
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputInlineQueryResultGame {
    /// Unique identifier of the query result
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// The message reply markup; pass null if none. Must be of type replyMarkupInlineKeyboard or null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
}

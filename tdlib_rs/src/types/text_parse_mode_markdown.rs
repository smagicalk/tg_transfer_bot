#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The text uses Markdown-style formatting
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextParseModeMarkdown {
    /// Version of the parser: 0 or 1 - Telegram Bot API "Markdown" parse mode, 2 - Telegram Bot API "MarkdownV2" parse mode
    pub version: i32,
}

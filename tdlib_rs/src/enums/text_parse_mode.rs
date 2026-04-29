#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextParseMode {
    /// The text uses Markdown-style formatting
    #[serde(rename(serialize = "textParseModeMarkdown", deserialize = "textParseModeMarkdown"))]
    Markdown(crate::types::TextParseModeMarkdown),
    /// The text uses HTML-style formatting. The same as Telegram Bot API "HTML" parse mode
    #[serde(rename(serialize = "textParseModeHTML", deserialize = "textParseModeHTML"))]
    Html,
}

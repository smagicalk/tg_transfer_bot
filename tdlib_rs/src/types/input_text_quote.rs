#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes manually chosen quote from another message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputTextQuote {
    /// Text of the quote; 0-getOption("message_reply_quote_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed to be kept and must be kept in the quote
    pub text: crate::types::FormattedText,
    /// Quote position in the original message in UTF-16 code units
    pub position: i32,
}

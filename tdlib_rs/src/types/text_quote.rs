#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes manually or automatically chosen quote from another message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextQuote {
    /// Text of the quote. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities can be present in the text
    pub text: crate::types::FormattedText,
    /// Approximate quote position in the original message in UTF-16 code units as specified by the message sender
    pub position: i32,
    /// True, if the quote was manually chosen by the message sender
    pub is_manual: bool,
}

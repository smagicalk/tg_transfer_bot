#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A text message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageText {
    /// Formatted text to be sent; 0-getOption("message_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, BlockQuote, ExpandableBlockQuote,
    /// Code, Pre, PreCode, TextUrl and MentionName entities are allowed to be specified manually
    pub text: crate::types::FormattedText,
    /// Options to be used for generation of a link preview; may be null if none; pass null to use default link preview options
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
    /// True, if the chat message draft must be deleted
    pub clear_draft: bool,
}

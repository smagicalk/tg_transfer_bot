#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeMessageDraft {
    /// Message draft text
    pub text: crate::types::FormattedText,
    /// True, if the first line of the text contains a link. If true, the input field needs to be focused and the text after the link must be selected
    pub contains_link: bool,
}

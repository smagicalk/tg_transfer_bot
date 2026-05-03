#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A text message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageText {
    /// Text of the message
    pub text: crate::types::FormattedText,
    /// A link preview attached to the message; may be null
    pub link_preview: Option<crate::types::LinkPreview>,
    /// Options which were used for generation of the link preview; may be null if default options were used
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Options to be used for generation of a link preview
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewOptions {
    /// True, if link preview must be disabled
    pub is_disabled: bool,
    /// URL to use for link preview. If empty, then the first URL found in the message text will be used
    pub url: String,
    /// True, if shown media preview must be small; ignored in secret chats or if the URL isn't explicitly specified
    pub force_small_media: bool,
    /// True, if shown media preview must be large; ignored in secret chats or if the URL isn't explicitly specified
    pub force_large_media: bool,
    /// True, if link preview must be shown above message text; otherwise, the link preview will be shown below the message text; ignored in secret chats
    pub show_above_text: bool,
}

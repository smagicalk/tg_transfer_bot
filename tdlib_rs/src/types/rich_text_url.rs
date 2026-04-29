#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A rich text URL link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextUrl {
    /// Text
    pub text: crate::enums::RichText,
    /// URL
    pub url: String,
    /// True, if the URL has cached instant view server-side
    pub is_cached: bool,
}

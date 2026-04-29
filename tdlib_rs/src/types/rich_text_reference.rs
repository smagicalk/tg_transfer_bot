#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A reference to a richTexts object on the same page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextReference {
    /// The text
    pub text: crate::enums::RichText,
    /// The name of a richTextAnchor object, which is the first element of the target richTexts object
    pub anchor_name: String,
    /// An HTTP URL, opening the reference
    pub url: String,
}

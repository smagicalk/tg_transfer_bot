#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A link to an anchor on the same page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextAnchorLink {
    /// The link text
    pub text: crate::enums::RichText,
    /// The anchor name. If the name is empty, the link must bring back to top
    pub anchor_name: String,
    /// An HTTP URL, opening the anchor
    pub url: String,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a caption of another block
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockCaption {
    /// Content of the caption
    pub text: crate::enums::RichText,
    /// Block credit (like HTML tag <cite>)
    pub credit: crate::enums::RichText,
}

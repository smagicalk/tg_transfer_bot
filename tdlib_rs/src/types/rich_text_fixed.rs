#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A fixed-width rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextFixed {
    /// Text
    pub text: crate::enums::RichText,
}

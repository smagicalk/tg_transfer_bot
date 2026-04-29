#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A plain text
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichTextPlain {
    /// Text
    pub text: String,
}

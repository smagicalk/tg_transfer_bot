#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A button that copies specified text to clipboard
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeCopyText {
    /// The text to copy to clipboard
    pub text: String,
}

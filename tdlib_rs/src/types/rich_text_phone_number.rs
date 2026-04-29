#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A rich text phone number
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextPhoneNumber {
    /// Text
    pub text: crate::enums::RichText,
    /// Phone number
    pub phone_number: String,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a Telegram Passport element that was requested by a service
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PassportSuitableElement {
    /// Type of the element
    pub r#type: crate::enums::PassportElementType,
    /// True, if a selfie is required with the identity document
    pub is_selfie_required: bool,
    /// True, if a certified English translation is required with the document
    pub is_translation_required: bool,
    /// True, if personal details must include the user's name in the language of their country of residence
    pub is_native_name_required: bool,
}

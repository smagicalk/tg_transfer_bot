#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes settings for a business account start page to set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputBusinessStartPage {
    /// Title text of the start page; 0-getOption("business_start_page_title_length_max") characters
    pub title: String,
    /// Message text of the start page; 0-getOption("business_start_page_message_length_max") characters
    pub message: String,
    /// Greeting sticker of the start page; pass null if none. The sticker must belong to a sticker set and must not be a custom emoji
    pub sticker: Option<crate::enums::InputFile>,
}

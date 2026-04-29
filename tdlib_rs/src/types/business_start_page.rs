#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes settings for a business account start page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessStartPage {
    /// Title text of the start page
    pub title: String,
    /// Message text of the start page
    pub message: String,
    /// Greeting sticker of the start page; may be null if none
    pub sticker: Option<crate::types::Sticker>,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An embedded web page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockEmbedded {
    /// URL of the embedded page, if available
    pub url: String,
    /// HTML-markup of the embedded page
    pub html: String,
    /// Poster photo, if available; may be null
    pub poster_photo: Option<crate::types::Photo>,
    /// Block width; 0 if unknown
    pub width: i32,
    /// Block height; 0 if unknown
    pub height: i32,
    /// Block caption
    pub caption: crate::types::PageBlockCaption,
    /// True, if the block must be full width
    pub is_full_width: bool,
    /// True, if scrolling needs to be allowed
    pub allow_scrolling: bool,
}

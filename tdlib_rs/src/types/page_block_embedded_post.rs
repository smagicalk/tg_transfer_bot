#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An embedded post
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockEmbeddedPost {
    /// URL of the embedded post
    pub url: String,
    /// Post author
    pub author: String,
    /// Post author photo; may be null
    pub author_photo: Option<crate::types::Photo>,
    /// Point in time (Unix timestamp) when the post was created; 0 if unknown
    pub date: i32,
    /// Post content
    pub page_blocks: Vec<crate::enums::PageBlock>,
    /// Post caption
    pub caption: crate::types::PageBlockCaption,
}

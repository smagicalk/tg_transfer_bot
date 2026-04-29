#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an instant view page for a web page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WebPageInstantView {
    /// Content of the instant view page
    pub page_blocks: Vec<crate::enums::PageBlock>,
    /// Number of the instant view views; 0 if unknown
    pub view_count: i32,
    /// Version of the instant view; currently, can be 1 or 2
    pub version: i32,
    /// True, if the instant view must be shown from right to left
    pub is_rtl: bool,
    /// True, if the instant view contains the full page. A network request might be needed to get the full instant view
    pub is_full: bool,
    /// An internal link to be opened to leave feedback about the instant view
    pub feedback_link: crate::enums::InternalLinkType,
}

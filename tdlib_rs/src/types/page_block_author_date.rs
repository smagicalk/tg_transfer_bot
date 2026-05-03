#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The author and publishing date of a page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockAuthorDate {
    /// Author
    pub author: crate::enums::RichText,
    /// Point in time (Unix timestamp) when the article was published; 0 if unknown
    pub publish_date: i32,
}

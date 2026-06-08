#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a related article
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockRelatedArticle {
    /// Related article URL
    pub url: String,
    /// Article title; may be empty
    pub title: String,
    /// Article description; may be empty
    pub description: String,
    /// Article photo; may be null
    pub photo: Option<crate::types::Photo>,
    /// Article author; may be empty
    pub author: String,
    /// Point in time (Unix timestamp) when the article was published; 0 if unknown
    pub publish_date: i32,
}

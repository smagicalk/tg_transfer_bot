#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a link to an article or web page
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultArticle {
    /// Unique identifier of the query result
    pub id: String,
    /// URL of the result, if it exists
    pub url: String,
    /// Title of the result
    pub title: String,
    /// A short description of the result
    pub description: String,
    /// Result thumbnail in JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
}

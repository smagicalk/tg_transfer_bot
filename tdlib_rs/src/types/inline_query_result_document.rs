#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a document
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultDocument {
    /// Unique identifier of the query result
    pub id: String,
    /// Document
    pub document: crate::types::Document,
    /// Document title
    pub title: String,
    /// Document description
    pub description: String,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a photo
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultPhoto {
    /// Unique identifier of the query result
    pub id: String,
    /// Photo
    pub photo: crate::types::Photo,
    /// Title of the result, if known
    pub title: String,
    /// A short description of the result, if known
    pub description: String,
}

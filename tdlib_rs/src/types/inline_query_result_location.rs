#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a point on the map
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultLocation {
    /// Unique identifier of the query result
    pub id: String,
    /// Location result
    pub location: crate::types::Location,
    /// Title of the result
    pub title: String,
    /// Result thumbnail in JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
}

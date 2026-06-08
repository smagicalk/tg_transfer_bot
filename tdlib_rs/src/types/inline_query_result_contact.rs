#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a user contact
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultContact {
    /// Unique identifier of the query result
    pub id: String,
    /// A user contact
    pub contact: crate::types::Contact,
    /// Result thumbnail in JPEG format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
}

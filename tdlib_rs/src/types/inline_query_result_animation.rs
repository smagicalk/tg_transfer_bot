#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents an animation file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultAnimation {
    /// Unique identifier of the query result
    pub id: String,
    /// Animation file
    pub animation: crate::types::Animation,
    /// Animation title
    pub title: String,
}

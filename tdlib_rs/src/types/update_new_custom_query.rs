#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new incoming query; for bots only
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCustomQuery {
    /// The query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// JSON-serialized query data
    pub data: String,
    /// Query timeout
    pub timeout: i32,
}

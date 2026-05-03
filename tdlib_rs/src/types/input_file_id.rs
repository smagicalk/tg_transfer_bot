#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A file defined by its unique identifier
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputFileId {
    /// Unique file identifier
    pub id: i32,
}

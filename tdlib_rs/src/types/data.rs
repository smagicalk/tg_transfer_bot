#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains some binary data
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Data {
    /// Data
    pub data: String,
}

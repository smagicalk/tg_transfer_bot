#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a counter
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Count {
    /// Count
    pub count: i32,
}

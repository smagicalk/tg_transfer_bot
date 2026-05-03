#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a value representing a number of seconds
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Seconds {
    /// Number of seconds
    pub seconds: f64,
}

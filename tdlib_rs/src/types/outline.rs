#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents outline of an image
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Outline {
    /// The list of closed vector paths
    pub paths: Vec<crate::types::ClosedVectorPath>,
}

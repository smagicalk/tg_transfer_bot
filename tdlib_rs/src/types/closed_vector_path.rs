#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a closed vector path. The path begins at the end point of the last command. The coordinate system origin is in the upper-left corner
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ClosedVectorPath {
    /// List of vector path commands
    pub commands: Vec<crate::enums::VectorPathCommand>,
}

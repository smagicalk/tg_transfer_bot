#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of animations
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Animations {
    /// List of animations
    pub animations: Vec<crate::types::Animation>,
}

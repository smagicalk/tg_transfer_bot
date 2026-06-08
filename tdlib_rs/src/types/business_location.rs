#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a location of a business
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessLocation {
    /// The location; may be null if not specified
    pub location: Option<crate::types::Location>,
    /// Location address; 1-96 characters
    pub address: String,
}

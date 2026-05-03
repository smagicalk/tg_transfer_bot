#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a JSON object
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueObject {
    /// The list of object members
    pub members: Vec<crate::types::JsonObjectMember>,
}

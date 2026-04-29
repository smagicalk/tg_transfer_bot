#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An area pointing to a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeLocation {
    /// The location
    pub location: crate::types::Location,
    /// Address of the location; may be null if unknown
    pub address: Option<crate::types::LocationAddress>,
}

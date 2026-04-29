#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of group call streams
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallStreams {
    /// A list of group call streams
    pub streams: Vec<crate::types::GroupCallStream>,
}

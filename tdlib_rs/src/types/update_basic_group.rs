#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateBasicGroup {
    /// New data about the group
    pub basic_group: crate::types::BasicGroup,
}

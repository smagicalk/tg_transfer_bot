#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The messages were exported from a group chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageFileTypeGroup {
    /// Title of the group chat; may be empty if unrecognized
    pub title: String,
}

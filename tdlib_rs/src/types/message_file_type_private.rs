#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The messages were exported from a private chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageFileTypePrivate {
    /// Name of the other party; may be empty if unrecognized
    pub name: String,
}

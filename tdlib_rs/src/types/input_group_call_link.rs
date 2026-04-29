#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The group call is accessible through a link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputGroupCallLink {
    /// The link for the group call
    pub link: String,
}

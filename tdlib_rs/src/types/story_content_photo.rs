#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A photo story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentPhoto {
    /// The photo
    pub photo: crate::types::Photo,
}

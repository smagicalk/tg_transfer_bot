#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An area pointing to a HTTP or tg: link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeLink {
    /// HTTP or tg: URL to be opened when the area is clicked
    pub url: String,
}

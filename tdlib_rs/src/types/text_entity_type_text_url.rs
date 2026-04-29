#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A text description shown instead of a raw URL
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeTextUrl {
    /// HTTP or tg: URL to be opened when the link is clicked
    pub url: String,
}

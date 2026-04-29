#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockAnchor {
    /// Name of the anchor
    pub name: String,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a background. Call searchBackground with the given background name to process the link.
/// If background is found and the user wants to apply it, then call setDefaultBackground
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBackground {
    /// Name of the background
    pub background_name: String,
}

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link must be opened in an Instant View. Call getWebPageInstantView with the given URL to process the link.
/// If Instant View is found, then show it, otherwise, open the fallback URL in an external browser
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeInstantView {
    /// URL to be passed to getWebPageInstantView
    pub url: String,
    /// An URL to open if getWebPageInstantView fails
    pub fallback_url: String,
}

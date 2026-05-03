#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to the Call tab or page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeCallsPage {
    /// Section of the page; may be one of
    /// "", "all", "missed", "edit", "show-tab", "start-call"
    pub section: String,
}

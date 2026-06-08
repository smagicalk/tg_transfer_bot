#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to the Contacts tab or page
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeContactsPage {
    /// Section of the page; may be one of
    /// "", "search", "sort", "new", "invite", "manage"
    pub section: String,
}
